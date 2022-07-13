use actix::Addr;
use actix_redis::{RedisActor, RespValue};
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
// use tokio_postgres::row::Row;
// use uuid::Uuid;
use tokio_pg_mapper_derive::PostgresMapper;
use tokio_postgres::Row;
use async_trait::async_trait;

use crate::{
    base::{redis_key::RedisKey, user_info::UserInfo, big_int::BigInt},
    utils::{db_helper::{RedisActorHelper, RedisCmd}}, traits::sync_cache::SyncCache,
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
    pub id: BigInt,
    pub sender: i32,
    pub content: String,
    pub create_time: NaiveDateTime,
    pub likes: i64,
    pub comments: i64,
    pub extends: Option<BigInt>,
}

impl From<&Row> for Post {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get("id"),
            sender: row.get("sender"),
            content: row.get("content"),
            create_time: row.get("create_time"),
            likes: row.get("likes"),
            comments: row.get("comments"),
            extends: row.get("extends"),
        }
    }
}


#[async_trait]
impl SyncCache for Post {
    /// 把redis上的数据合并
    /// 返回 true 表明 redis 上有对应数据
    /// 返回 false 则表明没有
    async fn sync_cache_data(&mut self, _user: &UserInfo, redis_addr: &Addr<RedisActor>) -> () {
        // 拉取redis里缓存的数量
        let id = self.id.inner();
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
    /// 这个字段留着后端用，前端需要用BigNumber，很麻烦...
    // #[serde(skip_serializing)]
    pub id: BigInt,
    pub nick: String,
    pub content: String,
    pub create_time: NaiveDateTime,
    pub like_count: i64,
    pub comment_count: i64,
    /// 我是否点赞过，从redis上获取
    pub liked: bool,
    /// 转发的id
    pub origin_id: Option<BigInt>,
    /// 转发的内容
    pub origin_content: Option<String>,
    /// 转发人的昵称
    pub origin_sender_nick: Option<String>,
    pub comments: Vec<Comment>,
}

impl PostExtends {
    /// 第一页最大的评论数量，剩下的翻页获取
    pub fn max_comments() -> &'static usize {
        static MAX_COMMENTS: usize = 10;
        &MAX_COMMENTS
    }
}

impl From<&Row> for PostExtends {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get("id"),
            nick: row.get("nick"),
            content: row.get("content"),
            create_time: row.get("create_time"),
            like_count: row.get("likes"),
            liked: false,
            comment_count: row.get("comments"),
            origin_id: row.get("origin_id"),
            origin_content: row.get("origin_content"),
            origin_sender_nick: row.get("origin_sender_nick"),
            comments: Vec::with_capacity(*Self::max_comments()),
        }
    }
}

#[async_trait]
impl SyncCache for PostExtends {
    /// 把redis上的数据合并
    /// 返回 true 表明 redis 上有对应数据
    /// 返回 false 则表明没有
    async fn sync_cache_data(&mut self, user: &UserInfo, redis_addr: &Addr<RedisActor>) -> () {
        // 拉取redis里缓存的数量
        let id = self.id.inner();
        // 点赞数量
        if let Ok(num) = redis_addr.get_i64(&RedisKey::post_like_count(id)).await {
            self.like_count = num;
        }
        // 评论数量
        if let Ok(num) = redis_addr.get_i64(&RedisKey::post_comments_count(id)).await {
            self.comment_count = num;
        }
        // 获取我是否点赞
        if let Ok(RespValue::Integer(num)) = redis_addr
            .exec(RedisCmd::sismember(
                &RedisKey::post_likes(id),
                &user.id.to_string(),
            ))
            .await
        {
            if num == 1 {
                // 已经点赞
                self.liked = true;
            }
        }
    }
}


#[derive(Debug, Deserialize, Serialize)]
pub struct Comment {
    pub id: BigInt,
    pub nick: String,
    pub content: String,
    pub create_time: NaiveDateTime,
    pub likes: i64,
    pub comments: i64,
    /// 我是否点赞过，从redis上获取
    pub liked: bool,
}

impl From<&Row> for Comment {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get("id"),
            nick: row.get("nick"),
            content: row.get("content"),
            create_time: row.get("create_time"),
            likes: row.get("likes"),
            liked: false,
            comments: row.get("comments"),
        }
    }
}

#[async_trait]
impl SyncCache for Comment {
    /// 把redis上的数据合并
    /// 返回 true 表明 redis 上有对应数据
    /// 返回 false 则表明没有
    async fn sync_cache_data(&mut self, _user: &UserInfo, redis_addr: &Addr<RedisActor>) -> () {
        // 拉取redis里缓存的数量
        let id = self.id.inner();
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