use actix::Addr;
use actix_redis::{RedisActor, RespValue};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use async_trait::async_trait;
use tokio_postgres::Row;

use crate::{
    base::{big_int::BigInt, redis_key::RedisKey, user_info::UserInfo},
    data_models::user::*,
    errors::MyError,
    traits::sync_cache::SyncCache,
    utils::db_helper::{RedisActorHelper, RedisCmd, RespValueRedisHelper},
};

#[derive(Debug, Deserialize, Serialize)]
pub struct PostExtends {
    /// 这个字段留着后端用，前端需要用BigNumber，很麻烦...
    // #[serde(skip_serializing)]
    pub id: BigInt,
    pub sender: UserData,
    pub content: String,
    pub create_time: NaiveDateTime,
    pub like_count: i32,
    pub comment_count: i32,
    pub hate_count: i32,
    /// 我是否点赞，从redis上获取
    pub liked: bool,
    /// 是否讨厌，从redis上获取
    pub hated: bool,
    /// 转发的id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_id: Option<BigInt>,
    /// 转发的内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_content: Option<String>,
    /// 转发人的昵称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_sender: Option<UserData>,
    /// 转发内容的创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_create_time: Option<NaiveDateTime>,
}

impl From<&Row> for PostExtends {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get("id"),
            sender: UserData::new(
                &row.get("sender_id"),
                &row.get("sender_nick"),
                row.try_get("sender_avatar_url").ok(),
            ),
            content: row.get("content"),
            create_time: row.get("create_time"),
            like_count: row.get("likes"),
            hate_count: row.get("hates"),
            comment_count: row.get("comments"),
            liked: false,
            hated: false,
            origin_id: row.try_get("origin_id").ok(),
            origin_content: row.try_get("origin_content").ok(),
            origin_sender: UserData::optional(
                &row.try_get("origin_sender_id").ok(),
                &row.try_get("origin_sender_nick").ok(),
                row.try_get("origin_sender_avatar_url").ok(),
            ),
            origin_create_time: row.try_get("origin_create_time").ok(),
        }
    }
}

#[async_trait]
impl SyncCache for PostExtends {
    /// 把redis上的数据合并
    /// 返回 true 表明 redis 上有对应数据
    /// 返回 false 则表明没有
    async fn sync_cache_data(
        &mut self,
        user: Option<&UserInfo>,
        redis_addr: &Addr<RedisActor>,
    ) -> Result<(), MyError> {
        // 拉取redis里缓存的数量
        let id = self.id.inner();
        let user_id = user.map(|v| &v.id);
        let have_user = user_id.is_some();
        let mut ret = redis_addr
            .exec_all({
                let mut vec = vec![
                    RedisCmd::get(&RedisKey::post_like_count(&id)),
                    RedisCmd::get(&RedisKey::post_hate_count(&id)),
                ];
                if have_user {
                    let user_id = user_id.unwrap().to_string();
                    vec.append(&mut vec![
                        // 获取我是否点赞
                        RedisCmd::sismember(&RedisKey::post_likes(id), &user_id),
                        // 获取我是否反感
                        RedisCmd::sismember(&RedisKey::post_hates(id), &user_id),
                    ]);
                }
                vec
            })
            .await?
            .into_iter();

        if let Some(val) = ret.next() {
            if let Some(num) = val.bulk_to_num() {
                self.like_count = num;
            }
        }
        if let Some(val) = ret.next() {
            if let Some(num) = val.bulk_to_num() {
                self.hate_count = num;
            }
        }
        if have_user {
            if let Some(RespValue::Integer(num)) = ret.next() {
                if num == 1 {
                    // 已经点赞
                    self.liked = true;
                }
            }
            if let Some(RespValue::Integer(num)) = ret.next() {
                if num == 1 {
                    // 已经反感
                    self.hated = true;
                }
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


