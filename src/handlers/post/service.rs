use actix::Addr;
use actix_redis::RedisActor;

use crate::{
    base::{big_int::BigInt, pg_client::PGClient, redis_key::RedisKey},
    errors::MyError, utils::db_helper::{RedisActorHelper, RespValueRedisHelper, RedisCmd},
};

/// 根据postid 获取 发送者id
pub async fn get_post_sender_from_id(
    post_id: &BigInt,
    client: &PGClient,
    redis_addr: &Addr<RedisActor>,
) -> Result<i32, MyError> {
    let result = redis_addr.exec(RedisCmd::get(&RedisKey::post_sender(&post_id))).await;
    if let Ok(resp_value) = result {
        let result = resp_value.bulk_to_num::<i32>();
        if let Some(id) = result {
            return Ok(id);
        }
    }

    let _stmt = include_str!("../../../sql/post/get_sender.sql");
    let stmt = client.prepare(_stmt).await?;
    let result = client
        .query(&stmt, &[&post_id])
        .await?
        .iter()
        .map(|row| row.get("sender"))
        .collect::<Vec<i32>>()
        .pop();
    if let Some(id) = result {
        return Ok(id);
    }
    Err(MyError::NotFound)
}
