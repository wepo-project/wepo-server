use actix::Addr;
use actix_redis::RedisActor;

use crate::{
    base::{big_int::BigInt, pg_client::PGClient},
    errors::MyError,
};

use super::storage;

pub async fn get_post_sender_from_id(
    post_id: &BigInt,
    client: &PGClient,
    redis_addr: &Addr<RedisActor>,
) -> Result<i32, MyError> {
    storage::get_post_sender_from_id(post_id, client, redis_addr).await
}
