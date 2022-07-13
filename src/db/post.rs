use std::sync::Mutex;

use actix::Addr;
use actix_redis::{RedisActor, RespValue};
use deadpool_postgres::Client;
use futures::future::join_all;
use log::info;
use once_cell::sync::Lazy;
use snowflake::SnowflakeIdBucket;
use tokio_pg_mapper::FromTokioPostgresRow;

use crate::{
    base::{redis_key::RedisKey, user_info::UserInfo},
    data_models::{Post, PostExtends},
    errors::MyError,
    models::post::dto::*,
    traits::sync_cache::SyncCache,
    utils::{
        self,
        db_helper::{RedisActorHelper, RedisCmd, RespValueRedisHelper},
    },
};

/// 雪花id生成器
static POST_ID_BUCKET: Lazy<Mutex<SnowflakeIdBucket>> =
    Lazy::new(|| Mutex::new(SnowflakeIdBucket::new(1, 1)));

fn get_next_id() -> Result<i64, MyError> {
    Ok(POST_ID_BUCKET
        .lock()
        .map_err(|_| MyError::PoisonError)?
        .get_id())
}

/// 添加
pub async fn add_post(
    user: &UserInfo,
    post_data: &AddPostDTO,
    client: &Client,
) -> Result<i64, MyError> {
    let _stmt = include_str!("../../sql/post/add.sql");
    // let _stmt = _stmt.replace("$table_fields", &Post::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.map_err(MyError::PGError)?;
    let post_id = get_next_id()?;
    client
        .query(&stmt, &[&post_id, &user.id, &post_data.content])
        .await?
        .iter()
        .map(|row| row.get("id"))
        .collect::<Vec<i64>>()
        .pop()
        .ok_or(MyError::NotFound)
}

/// 删除推文
pub async fn del_post(
    user: &UserInfo,
    del_data: &DelPostDTO,
    client: &Client,
    redis_addr: &Addr<RedisActor>,
) -> Result<(), MyError> {
    let _stmt = include_str!("../../sql/post/delete.sql");
    let stmt = client.prepare(_stmt).await.map_err(MyError::PGError)?;

    client
        .query(&stmt, &[&del_data.id, &user.id])
        .await
        .map(|_| {
            // 删除post的redis缓存数据
            redis_addr.del(&RedisKey::post_likes(&del_data.id));
            redis_addr.del(&RedisKey::post_like_count(&del_data.id));
        })
        .map_err(MyError::PGError)
}

/// 获取某个推文
pub async fn get_post(
    user: &UserInfo,
    post_id: &i64,
    client: &Client,
    redis_addr: &Addr<RedisActor>,
) -> Result<PostExtends, MyError> {
    let _stmt = include_str!("../../sql/post/get.sql");
    let stmt = client.prepare(_stmt).await.map_err(MyError::PGError)?;
    let mut post_ext = client
        .query(&stmt, &[&post_id])
        .await?
        .iter()
        .map(|row| PostExtends::from(row))
        .collect::<Vec<PostExtends>>()
        .pop()
        .ok_or(MyError::NotFound)?;

    info!("{:?}", post_ext);

    // 如果 同步失败，则直接返回
    let _ = post_ext.sync_cache_data(redis_addr).await;

    // 获取我是否点赞
    if let Ok(RespValue::Integer(num)) = redis_addr
        .exec(RedisCmd::sismember(
            &RedisKey::post_likes(post_id),
            &user.id.to_string(),
        ))
        .await
    {
        if num == 1 {
            // 已经点赞
            post_ext.liked = true;
        }
    }

    Ok(post_ext)
}

/// 点赞
/// 用户点赞 数据结构
/// key: post_like:<post_id>
/// value: {user_id}
/// 点赞数
/// key: post_like_count:<post_id>
/// value: likes count
pub async fn like_post(
    post_id: &i64,
    user_id: &i32,
    redis_addr: &Addr<RedisActor>,
) -> Result<(), MyError> {
    let likes_key = RedisKey::post_likes(post_id);
    // 判断是否重复点赞
    let liked = redis_addr
        .exec(RedisCmd::sismember(&likes_key, &user_id.to_string()))
        .await?;
    if liked.int_to_bool() {
        // 已经点赞
        return Err(MyError::code(201));
    }

    redis_addr
        .exec_all(vec![
            // 添加进点赞集合
            RedisCmd::sadd(&likes_key, &user_id.to_string()),
            // 增加点赞数
            RedisCmd::incr(&RedisKey::post_like_count(post_id)),
        ])
        .await?;
    Ok(())
}

/// 取消点赞
pub async fn unlike_post(
    post_id: &i64,
    user_id: &i32,
    redis_addr: &Addr<RedisActor>,
) -> Result<(), MyError> {
    let likes_key = RedisKey::post_likes(post_id);

    let liked = redis_addr
        .exec(RedisCmd::sismember(&likes_key, &user_id.to_string()))
        .await?;

    if !liked.int_to_bool() {
        // 没有点赞，取消点赞则返回
        return Err(MyError::code(201));
    }

    redis_addr
        .exec_all(vec![
            // 移除出点赞集合
            RedisCmd::srem(&likes_key, &user_id.to_string()),
            // 减少点赞数
            RedisCmd::decr(&RedisKey::post_like_count(post_id)),
        ])
        .await?;
    Ok(())
}

/// 查看我的post
pub async fn get_my_posts(
    user_id: &i32,
    page: &i64,
    limit: &i64,
    client: &Client,
    redis_addr: &Addr<RedisActor>,
) -> Result<Vec<Post>, MyError> {
    let _stmt = include_str!("../../sql/post/get_list.sql");
    let stmt = client.prepare(_stmt).await.map_err(MyError::PGError)?;
    let offset = limit * page;
    let vec = client
        .query(&stmt, &[user_id, &limit, &offset])
        .await
        .unwrap();

    Ok(join_all(vec.iter().map(|row| async move {
        // move 把row引用带出闭包
        let mut post = Post::from(row);
        let _ = post.sync_cache_data(redis_addr).await;
        post
    }))
    .await)
}

/// 评论
pub async fn comment_post(
    user: &UserInfo,
    data: &CommentPostDTO,
    client: &Client,
    redis_addr: &Addr<RedisActor>,
) -> Result<i64, MyError> {
    let _stmt = include_str!("../../sql/post/comment.sql");
    let _stmt = _stmt.replace("$table_fields", &Post::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.map_err(MyError::PGError)?;
    let post_id = get_next_id()?;
    let origin_id: i64 = utils::string_to_i64(&data.origin_id);
    // 插入一条数据
    let pg_ret = client
        .query(&stmt, &[&post_id, &user.id, &data.content, &origin_id])
        .await?;

    // 评论成功 修改原本的post信息
    let _ret = redis_addr
        .exec_all(vec![
            // 标记为评论
            RedisCmd::lpush(&RedisKey::post_comments(&origin_id), &post_id.to_string()),
            // 增加原po的评论数
            RedisCmd::incr(&RedisKey::post_comments_count(&origin_id)),
        ])
        .await;

    pg_ret
        .iter()
        .map(|row| row.get("id"))
        .collect::<Vec<i64>>()
        .pop()
        .ok_or(MyError::NotFound)
}
