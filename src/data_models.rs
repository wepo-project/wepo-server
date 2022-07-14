use actix::Addr;
use actix_redis::{RedisActor, RespValue};
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
// use tokio_postgres::row::Row;
// use uuid::Uuid;
use async_trait::async_trait;
use tokio_pg_mapper_derive::PostgresMapper;
use tokio_postgres::Row;

use crate::{
    base::{big_int::BigInt, redis_key::RedisKey, user_info::UserInfo},
    errors::MyError,
    traits::sync_cache::SyncCache,
    utils::db_helper::{RedisActorHelper, RedisCmd, RespValueRedisHelper},
};

#[derive(Debug, Deserialize, Serialize, PostgresMapper)]
#[pg_mapper(table = "users")]
pub struct User {
    pub id: i32,
    pub nick: String,
    pub pwd: Option<String>,
    pub _salt: String,
    pub avatar_url: Option<String>, // https://avatars.dicebear.com/api/pixel-art-neutral/123.svg
    pub create_time: NaiveDate,
}

impl User {
    pub fn get_avatar_url(nick: &String) -> String {
        format!("https://avatars.dicebear.com/api/{}/{}.svg", "pixel-art-neutral", nick)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PostSender {
    pub id: i32,
    pub nick: String,
    pub avatar_url: String,
}

impl PostSender {
    pub fn new(id: &i32, nick: &String) -> Self {
        Self {
            id: *id,
            nick: nick.clone(),
            avatar_url: User::get_avatar_url(&nick),
        }
    }
    pub fn optional(id: &Option<i32>, nick: &Option<String>) -> Option<Self> {
        if let Some(id) = id {
            if let Some(nick) = nick {
                return Some(Self::new(id, nick));
            }
        }
        None
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PostExtends {
    /// 这个字段留着后端用，前端需要用BigNumber，很麻烦...
    // #[serde(skip_serializing)]
    pub id: BigInt,
    pub sender: PostSender,
    pub content: String,
    pub create_time: NaiveDateTime,
    pub like_count: i64,
    pub comment_count: i64,
    /// 我是否点赞过，从redis上获取
    pub liked: bool,
    /// 转发的id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_id: Option<BigInt>,
    /// 转发的内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_content: Option<String>,
    /// 转发人的昵称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_sender: Option<PostSender>,
    /// 转发内容的创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_create_time: Option<NaiveDateTime>,
}

impl From<&Row> for PostExtends {

    fn from(row: &Row) -> Self {
        Self {
            id: row.get("id"),
            sender: PostSender::new(&row.get("sender_id"), &row.get("sender_nick")),
            content: row.get("content"),
            create_time: row.get("create_time"),
            like_count: row.get("likes"),
            liked: false,
            comment_count: row.get("comments"),
            origin_id: row.try_get("origin_id").ok(), 
            origin_content: row.try_get("origin_content").ok(),
            origin_sender: PostSender::optional(&row.try_get("origin_sender_id").ok(), &row.try_get("origin_sender_nick").ok()),
            origin_create_time: row.try_get("origin_create_time").ok(),
        }
    }
}

// impl TryFrom<&Row> for PostExtends {
//     type Error = MyError;

//     fn try_from(row: &Row) -> Result<Self, Self::Error> {
//         Ok(Self {
//             id: row.try_get("id")?,
//             nick: row.try_get("nick")?,
//             content: row.try_get("content")?,
//             create_time: row.try_get("create_time")?,
//             like_count: row.try_get("likes")?,
//             liked: false,
//             comment_count: row.try_get("comments")?,
//             origin_id: row.try_get("origin_id")?,
//             origin_content: row.try_get("origin_content")?,
//             origin_sender_nick: row.try_get("origin_sender_nick")?,
//             origin_create_time: row.try_get("origin_create_time")?,
//         })
//     }
// }

#[async_trait]
impl SyncCache for PostExtends {
    /// 把redis上的数据合并
    /// 返回 true 表明 redis 上有对应数据
    /// 返回 false 则表明没有
    async fn sync_cache_data(
        &mut self,
        user: &UserInfo,
        redis_addr: &Addr<RedisActor>,
    ) -> Result<(), MyError> {
        // 拉取redis里缓存的数量
        let id = self.id.inner();
        let mut ret = redis_addr
            .exec_all(vec![
                RedisCmd::get(&RedisKey::post_like_count(&id)),
                RedisCmd::get(&RedisKey::post_comments_count(&id)),
                // 获取我是否点赞
                RedisCmd::sismember(&RedisKey::post_likes(id), &user.id.to_string()),
            ])
            .await?
            .into_iter();

        if let Some(val) = ret.next() {
            if let Some(num) = val.bulk_to_i64() {
                self.like_count = num;
            }
        }
        if let Some(val) = ret.next() {
            if let Some(num) = val.bulk_to_i64() {
                self.comment_count = num;
            }
        }
        if let Some(RespValue::Integer(num)) = ret.next() {
            if num == 1 {
                // 已经点赞
                self.liked = true;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PostExtendsWithComment {
    pub post: PostExtends,
    pub comments: Vec<PostExtends>,
}

impl PostExtendsWithComment {
    pub fn from_post_ext(post: PostExtends) -> Self {
        Self {
            post,
            comments: Vec::with_capacity(Self::max_comments()),
        }
    }
    /// 第一页最大的评论数量，剩下的翻页获取
    pub fn max_comments() -> usize {
        static MAX_COMMENTS: usize = 10;
        MAX_COMMENTS
    }
}
