use actix::Addr;
use actix_redis::RedisActor;
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
// use tokio_postgres::row::Row;
// use uuid::Uuid;
use tokio_pg_mapper_derive::PostgresMapper;
use tokio_postgres::Row;
use async_trait::async_trait;

use crate::{
    base::redis_key::RedisKey,
    utils::{db_helper::{RedisActorHelper, RedisCmd}, self}, traits::sync_cache::SyncCache,
};

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
    pub id: String,
    pub sender: i32,
    pub content: String,
    pub create_time: NaiveDateTime,
    pub likes: i64,
    pub comments: i64,
    pub extends: Option<String>,
}

impl From<&Row> for Post {
    fn from(row: &Row) -> Self {
        let id: i64 = row.get("id");
        let extends: Option<i64> = row.get("extends");
        Self {
            id: id.to_string(),
            sender: row.get("sender"),
            content: row.get("content"),
            create_time: row.get("create_time"),
            likes: row.get("likes"),
            comments: row.get("comments"),
            extends: extends.map(|id| id.to_string()),
        }
    }
}


#[async_trait]
impl SyncCache for Post {
    /// 把redis上的数据合并
    /// 返回 true 表明 redis 上有对应数据
    /// 返回 false 则表明没有
    async fn sync_cache_data(&mut self, redis_addr: &Addr<RedisActor>) -> () {
        // 拉取redis里缓存的数量
        let id = utils::string_to_i64(&self.id);
        // 点赞
        if let Ok(num) = redis_addr.get_i64(&RedisKey::post_like_count(&id)).await {
            self.likes = num;
        }
        // 评论
        if let Ok(num) = redis_addr.get_i64(&RedisKey::post_comments_count(&id)).await {
            self.comments = num;
        }
    }
}


#[derive(Debug, Deserialize, Serialize)]
pub struct PostExtends {
    pub id: String,
    pub nick: String,
    pub content: String,
    pub create_time: NaiveDateTime,
    pub likes: i64,
    pub comments: i64,
    /// 我是否点赞过，从redis上获取
    pub liked: bool,
    /// 转发的id
    pub origin_id: Option<String>,
    /// 转发的内容
    pub origin_content: Option<String>,
    /// 转发人的昵称
    pub origin_sender_nick: Option<String>,
}

impl From<&Row> for PostExtends {
    fn from(row: &Row) -> Self {
        // id需要转成字符串
        let id: i64 = row.get("id");
        let origin_id: Option<i64> = row.get("origin_id");
        Self {
            id: id.to_string(),
            nick: row.get("nick"),
            content: row.get("content"),
            create_time: row.get("create_time"),
            likes: row.get("likes"),
            liked: false,
            comments: row.get("comments"),
            origin_id: origin_id.map(|id| id.to_string()),
            origin_content: row.get("origin_content"),
            origin_sender_nick: row.get("origin_sender_nick"),
        }
    }
}

#[async_trait]
impl SyncCache for PostExtends {
    /// 把redis上的数据合并
    /// 返回 true 表明 redis 上有对应数据
    /// 返回 false 则表明没有
    async fn sync_cache_data(&mut self, redis_addr: &Addr<RedisActor>) -> () {
        // 拉取redis里缓存的数量
        let id = &utils::string_to_i64(&self.id);
        // 点赞数量
        if let Ok(num) = redis_addr.get_i64(&RedisKey::post_like_count(id)).await {
            self.likes = num;
        }
        // 评论数量
        if let Ok(num) = redis_addr.get_i64(&RedisKey::post_comments_count(id)).await {
            self.comments = num;
        }
    }
}