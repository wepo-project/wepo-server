use actix::Addr;
use actix_redis::{resp_array, Command, RedisActor, RespValue};
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
// use tokio_postgres::row::Row;
// use uuid::Uuid;
use tokio_pg_mapper_derive::PostgresMapper;
use uuid::Uuid;

use crate::{base::redis_key::PostRedisKey, errors::MyError};

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
    pub likes: i32,
    // pub comments: i32,
}

impl Post {
    /// 把redis上的数据合并
    /// 返回 true 表明 redis 上有对应数据
    /// 返回 false 则表明没有
    pub async fn sync_cache_data(
        &mut self,
        redis_addr: &Addr<RedisActor>,
    ) -> Result<bool, MyError> {
        let key = PostRedisKey::new(&self.id);
        let val = redis_addr
            .send(Command(resp_array!["GET", &key.likes_count]))
            .await
            .map_err(MyError::MailboxError)?
            .map_err(MyError::RedisError)?;

        if let RespValue::Integer(num) = val {
            self.likes = num as i32;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
