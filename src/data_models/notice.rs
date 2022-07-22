
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;
use tokio_postgres::Row;

use crate::{base::big_int::BigInt, define_num_enum};

use super::user::UserData;

#[derive(Debug, Deserialize, Serialize, PostgresMapper)]
#[pg_mapper(table = "notices")]
pub struct Notice {
    /// 通知id
    pub id: BigInt,
    /// 发送者
    pub sender: i32,
    /// 通知类型
    pub notice_type: i16,
    /// 发送的对象主体ID
    pub sender_obj_id: String,
    /// 接收者的ID
    pub addressee_id: i32,
    /// 创建时间
    pub create_time: NaiveDateTime,
    /// 是否已读
    pub read: bool,
}

define_num_enum!{
    /// 通知类型
    NoticeType {
        /// 一个人评论了你, 评论的id
        [Comment => 1],
        /// 点赞通知
        [Like => 2],
        /// 点赞通知
        [Hate => 3],
    }
}



/// 评论通知
#[derive(Serialize, Deserialize)]
pub struct NoticeComment {
    pub id: BigInt,
    /// 评论者信息
    pub sender: UserData,
    /// 评论id
    pub post_id: BigInt,
    /// 评论内容
    pub content: String,
    /// 评论时间
    pub create_time: NaiveDateTime,
    /// 已读
    pub read: bool,
    /// 原文id
    pub origin_id: BigInt,
    /// 原文
    pub origin: String,
    /// 原文时间
    pub origin_create_time: NaiveDateTime,
}

impl From<&Row> for NoticeComment {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get("id"),
            sender: UserData::unreference(
                &row.get("sender_id"),
                row.get("sender_nick"),
                row.get("sender_avatar_url"),
            ),
            post_id: row.get("sender_obj_id"),
            content: row.get("content"),
            origin_id: row.get("origin_id"),
            origin: row.get("origin"),
            create_time: row.get("create_time"),
            read: row.get("read"),
            origin_create_time: row.get("origin_create_time"),
        }
    }
}



/// 评论通知
#[derive(Serialize, Deserialize)]
pub struct NoticePost {
    pub id: BigInt,
    /// 点赞者信息
    pub sender: UserData,
    /// 文章id
    pub post_id: BigInt,
    /// 文章内容
    pub content: String,
    /// 点赞时间
    pub create_time: NaiveDateTime,
    /// 已读
    pub read: bool,
}

impl From<&Row> for NoticePost {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get("id"),
            sender: UserData::unreference(
                &row.get("sender_id"),
                row.get("sender_nick"),
                row.get("sender_avatar_url"),
            ),
            post_id: row.get("sender_obj_id"),
            content: row.get("content"),
            create_time: row.get("create_time"),
            read: row.get("read"),
        }
    }
}