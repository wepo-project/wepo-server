use actix::Addr;
use actix_redis::RedisActor;
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
// use tokio_postgres::row::Row;
// use uuid::Uuid;
use tokio_pg_mapper_derive::PostgresMapper;
use uuid::Uuid;

use crate::{base::redis_key::PostRedisKey, utils::db_helper::RedisActorHelper};

#[derive(Debug, Deserialize, Serialize, PostgresMapper)]
#[pg_mapper(table = "users")]
pub struct User {
    pub id: i32,
    pub nick: String,
    pub pwd: Option<String>,
    pub _salt: String,
    pub create_time: NaiveDate,
}

#[derive(Debug, Deserialize, Serialize, PostgresMapper)]
#[pg_mapper(table = "posts")]
pub struct Post {
    pub id: Uuid,
    pub sender: i32,
    pub content: String,
    pub create_time: NaiveDateTime,
    pub likes: i64,
    pub comments: i64,
    pub reposts: i64,
}

impl Post {
    /// 把redis上的数据合并
    /// 返回 true 表明 redis 上有对应数据
    /// 返回 false 则表明没有
    pub async fn sync_cache_data(
        &mut self,
        redis_addr: &Addr<RedisActor>,
    ) -> bool {
        let key = PostRedisKey::new(&self.id);
        let like = redis_addr.get_i64(&key.likes_count).await;
        if let Ok(num) = like {
            self.likes = num;
        }
        like.is_ok()
    }
}
