use std::sync::Mutex;

use actix::Addr;
use actix_redis::{RedisActor, resp_array, RespValue};
use futures::future::{join_all, try_join_all};
use log::info;
use once_cell::sync::Lazy;
use serde::Serialize;
use snowflake::SnowflakeIdBucket;

use crate::{
    base::{
        big_int::BigInt, paging_data::Paging, pg_client::PGClient, redis_key::RedisKey,
        user_info::UserInfo,
    },
    data_models::post::{PostExtends, PostExtendsWithComment},
    errors::MyError,
    handlers::post::{data::CommentResult, dto::*},
    traits::sync_cache::SyncCache,
    utils::db_helper::{RedisActorHelper, RedisCmd, RespValueRedisHelper},
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
pub async fn add(
    user: &UserInfo,
    post_data: &AddPostDTO,
    client: &PGClient,
    redis_addr: &Addr<RedisActor>,
) -> Result<BigInt, MyError> {
    let _stmt = include_str!("../../../sql/post/add.sql");
    let stmt = client.prepare(&_stmt).await?;
    let post_id = get_next_id()?;
    let result = client
        .query(&stmt, &[&post_id, &user.id, &post_data.content])
        .await?
        .iter()
        .map(|row| row.get("id"))
        .collect::<Vec<BigInt>>()
        .pop()
        .ok_or(MyError::NotFound);

    if let Ok(id) = result {
        save_post_sender_cache(redis_addr, &id, &user.id);
    }

    result
}

/// 设置 postId -> userId 映射
pub fn save_post_sender_cache(redis_addr: &Addr<RedisActor>, post_id: &BigInt, user_id: &i32) {
    redis_addr.do_send(RedisCmd::set(
        &RedisKey::post_sender(&post_id),
        &user_id.to_string(),
    ));
    // redis_addr.do_send(RedisCmd::expire(
    //     &RedisKey::post_sender(&post_id),
    //     "604800", // 一周时间 过期
    // ));
}

/// 删除推文
/// 201 -> 没有权限删除
pub async fn delete(
    user: &UserInfo,
    del_data: &DelPostDTO,
    client: &PGClient,
    redis_addr: &Addr<RedisActor>,
) -> Result<(), MyError> {
    let _stmt = include_str!("../../../sql/post/delete.sql");
    let stmt = client.prepare(_stmt).await?;

    let vec = client.query(&stmt, &[&del_data.id, &user.id]).await?;

    // 返回条数 大于0 删除成功
    if vec.len() > 0 {
        // 删除post的redis缓存数据
        let id = &del_data.id;
        redis_addr.del(&RedisKey::post_likes(id)); // 删除赞集合
        redis_addr.del(&RedisKey::post_like_count(id)); // 删除赞数量
        redis_addr.del(&RedisKey::post_hates(id)); // 删除讨厌
        redis_addr.del(&RedisKey::post_hate_count(id)); // 删除讨厌数量
        Ok(())
    } else {
        Err(MyError::err_code(201))
    }
}

/// 获取某个推文
pub async fn get_one(
    user: &UserInfo,
    post_id: &i64,
    client: &PGClient,
    redis_addr: &Addr<RedisActor>,
) -> Result<impl Serialize, MyError> {
    let _stmt = include_str!("../../../sql/post/get.sql");
    let stmt = client.prepare(_stmt).await?;
    let mut post_ext = client
        .query(&stmt, &[&post_id])
        .await?
        .iter()
        .map(|row| PostExtends::from(row))
        .collect::<Vec<PostExtends>>()
        .pop()
        .ok_or(MyError::NotFound)?;

    info!("post_ext: {:?}", post_ext);

    // 同步
    let _ = post_ext.sync_cache_data(Some(user), redis_addr).await;

    // 获取评论
    let _stmt = include_str!("../../../sql/post/get_comments.sql");
    let stmt = client.prepare(_stmt).await?;
    let _skip = PostExtendsWithComment::max_comments() as i64;
    let _offset: i64 = 0;
    let mut comments = client
        .query(&stmt, &[&post_ext.id, &_skip, &_offset])
        .await?
        .iter()
        .map(|row| {
            info!("{:?}", row);
            PostExtends::from(row)
        })
        .collect::<Vec<PostExtends>>();

    // 同步每一条评论的点赞和评论数量
    let _result = try_join_all(
        comments
            .iter_mut()
            .map(|comment| comment.sync_cache_data(Some(user), redis_addr)),
    )
    .await;

    let mut data = PostExtendsWithComment::from_post_ext(post_ext);
    // 添加进之前的数组
    data.comments.append(&mut comments);

    Ok(data)
}

/// 点赞
pub async fn like(
    post_id: &i64,
    user_id: &i32,
    redis_addr: &Addr<RedisActor>,
) -> Result<(), MyError> {
    let likes_key = RedisKey::post_likes(post_id);
    // 判断是否重复点赞
    let liked = redis_addr
        .exec(RedisCmd::sismember(&likes_key, &user_id.to_string()))
        .await?;
    if liked.integer_to_bool() {
        // 已经点赞
        return Err(MyError::err_code(201));
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
pub async fn cancel_like(
    post_id: &i64,
    user_id: &i32,
    redis_addr: &Addr<RedisActor>,
) -> Result<(), MyError> {
    let likes_key = RedisKey::post_likes(post_id);

    let liked = redis_addr
        .exec(RedisCmd::sismember(&likes_key, &user_id.to_string()))
        .await?;

    if !liked.integer_to_bool() {
        // 没有点赞，取消点赞则返回
        return Err(MyError::err_code(201));
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
pub async fn get_mine<'a>(
    user: &UserInfo,
    paging: &Paging<'a>,
    client: &PGClient,
    redis_addr: &Addr<RedisActor>,
) -> Result<Vec<PostExtends>, MyError> {
    let _stmt = include_str!("../../../sql/post/get_list.sql");
    let stmt = client.prepare(_stmt).await?;
    let vec = client
        .query(&stmt, &[&user.id, paging.limit(), paging.offset()])
        .await?;

    Ok(join_all(vec.iter().map(|row| async move {
        // move 把row引用带出闭包
        let mut post = PostExtends::from(row);
        let _ = post.sync_cache_data(Some(user), redis_addr).await;
        post
    }))
    .await)
}

/// 评论
pub async fn comment(
    user: &UserInfo,
    data: &CommentPostDTO,
    client: &PGClient,
) -> Result<CommentResult, MyError> {
    let _stmt = include_str!("../../../sql/post/comment.sql");
    let stmt = client.prepare(&_stmt).await?;
    let post_id = get_next_id()?;

    client
        .query(&stmt, &[&post_id, &user.id, &data.content, &data.origin_id])
        .await?
        .iter()
        .map(|row| CommentResult::from(row))
        .collect::<Vec<CommentResult>>()
        .pop()
        .ok_or(MyError::NotFound)
}

/// 反感
pub async fn hate(
    post_id: &i64,
    user_id: &i32,
    redis_addr: &Addr<RedisActor>,
) -> Result<(), MyError> {
    let hate_key = RedisKey::post_hates(post_id);
    // 判断是否重复不喜欢
    let hated = redis_addr
        .exec(RedisCmd::sismember(&hate_key, &user_id.to_string()))
        .await?;
    if hated.integer_to_bool() {
        // 已经不喜欢
        return Err(MyError::err_code(201));
    }

    redis_addr
        .exec_all(vec![
            // 添加进不喜欢集合
            RedisCmd::sadd(&hate_key, &user_id.to_string()),
            // 增加讨厌数
            RedisCmd::incr(&RedisKey::post_hate_count(post_id)),
        ])
        .await?;
    Ok(())
}

/// 取消反感
pub async fn cancel_hate(
    post_id: &i64,
    user_id: &i32,
    redis_addr: &Addr<RedisActor>,
) -> Result<(), MyError> {
    let hate_key = RedisKey::post_hates(post_id);

    let hated = redis_addr
        .exec(RedisCmd::sismember(&hate_key, &user_id.to_string()))
        .await?;

    if !hated.integer_to_bool() {
        // 没有反感，返回
        return Err(MyError::err_code(201));
    }

    redis_addr
        .exec_all(vec![
            // 移除出反感集合
            RedisCmd::srem(&hate_key, &user_id.to_string()),
            // 减少反感数
            RedisCmd::decr(&RedisKey::post_hate_count(post_id)),
        ])
        .await?;
    Ok(())
}

/// 浏览
pub async fn browse<'a>(
    user: &UserInfo,
    client: &PGClient,
    paging: &Paging<'a>,
    redis_addr: &Addr<RedisActor>,
) -> Result<Vec<PostExtends>, MyError> {
    let _stmt = include_str!("../../../sql/post/browse.sql");
    let stmt = client.prepare(_stmt).await?;
    let vec = client
        .query(&stmt, &[paging.limit(), paging.offset()])
        .await?;

    Ok(join_all(vec.iter().map(|row| async move {
        let mut post = PostExtends::from(row);
        let _ = post.sync_cache_data(Some(user), redis_addr).await;
        post
    }))
    .await)
}
