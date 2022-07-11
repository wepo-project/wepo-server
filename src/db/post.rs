use actix::Addr;
use actix_redis::RedisActor;
use deadpool_postgres::Client;
use futures::future::join_all;
use log::info;
use tokio_pg_mapper::FromTokioPostgresRow;
use uuid::Uuid;

use crate::{
    base::{redis_key::PostRedisKey, user_info::UserInfo},
    data_models::Post,
    errors::MyError,
    models::post::dto::*, utils::db_helper::{RedisActorHelper, RedisCmd, RespValueRedisHelper},
};

/// 添加
pub async fn add_post(
    user: &UserInfo,
    post_data: &AddPostDTO,
    client: &Client,
) -> Result<Post, MyError> {
    let _stmt = include_str!("../../sql/post/add_post.sql");
    let _stmt = _stmt.replace("$table_fields", &Post::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();

    client
        .query(&stmt, &[&user.id, &post_data.content])
        .await?
        .iter()
        .map(|row| Post::from_row_ref(row).unwrap())
        .collect::<Vec<Post>>()
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
    let _stmt = include_str!("../../sql/post/delete_post.sql");
    let stmt = client.prepare(_stmt).await.unwrap();

    client
        .query(&stmt, &[&del_data.id, &user.id])
        .await
        .map(|_| {
            // 删除post的redis缓存数据
            let key = PostRedisKey::new(&del_data.id);
            redis_addr.del(&key.likes);
            redis_addr.del(&key.likes_count);
        })
        .map_err(MyError::PGError)
}

pub async fn get_post(
    post_id: &Uuid,
    client: &Client,
    redis_addr: &Addr<RedisActor>,
) -> Result<Post, MyError> {
    let _stmt = include_str!("../../sql/post/get_post.sql");
    let stmt = client.prepare(_stmt).await.map_err(MyError::PGError)?;
    let mut post = client
        .query(&stmt, &[&post_id.hyphenated().to_string()])
        .await?
        .iter()
        .map(|row| Post::from_row_ref(row).unwrap())
        .collect::<Vec<Post>>()
        .pop()
        .ok_or(MyError::NotFound)?;

    info!("{:?}", post);

    // 如果 同步失败，则直接返回
    let _synced = post.sync_cache_data(redis_addr).await;

    Ok(post)
}

/// 点赞
/// 用户点赞 数据结构
/// key: post_like:<post_id>
/// value: {user_id}
/// 点赞数
/// key: post_like_count:<post_id>
/// value: likes count
pub async fn like_post(
    post_id: &Uuid,
    user_id: &i32,
    redis_addr: &Addr<RedisActor>,
) -> Result<(), MyError> {
    let key = PostRedisKey::new(post_id);
    // 判断是否重复点赞
    let liked = redis_addr.exec(
        RedisCmd::sismember(&key.likes, &user_id.to_string())
    ).await?;
    if liked.int_to_bool() {
        // 重复点赞
        return Err(MyError::FailResultError);
    }

    redis_addr.exec_all(vec![
        // 添加进点赞集合
        RedisCmd::sadd(&key.likes, &user_id.to_string()),
        // 增加点赞数
        RedisCmd::incr(&key.likes_count),
    ]).await?;
    Ok(())
}

/// 取消点赞
pub async fn unlike_post(
    post_id: &Uuid,
    user_id: &i32,
    redis_addr: &Addr<RedisActor>,
) -> Result<(), MyError> {
    let key = PostRedisKey::new(post_id);

    let liked = redis_addr.exec(
        RedisCmd::sismember(&key.likes, &user_id.to_string())
    ).await?;

    if !liked.int_to_bool() {
        // 没有点赞，取消点赞则返回
        return Err(MyError::FailResultError);
    }

    redis_addr.exec_all(vec![
        // 移除出点赞集合
        RedisCmd::srem(&key.likes, &user_id.to_string()),
        // 减少点赞数
        RedisCmd::decr(&key.likes_count),
    ]).await?;
    Ok(())
}

/// 查看我的post
pub async fn get_my_post(
    user_id: &i32,
    page: &i64,
    limit: &i64,
    client: &Client,
    redis_addr: &Addr<RedisActor>,
) -> Result<Vec<Post>, MyError> {
    let _stmt = include_str!("../../sql/post/get_post_list.sql");
    let stmt = client.prepare(_stmt).await.map_err(MyError::PGError)?;
    let offset = limit * page;
    let vec = client
        .query(&stmt, &[user_id, &limit, &offset])
        .await
        .unwrap();

    Ok(join_all(vec.iter().map(|row| async {
        let mut post = Post::from_row_ref(row).unwrap();
        let _ = post.sync_cache_data(redis_addr).await;
        post
    }))
    .await)
}
