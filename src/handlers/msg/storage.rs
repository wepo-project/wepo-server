use actix::Addr;
use actix_redis::RedisActor;
use log::info;

use crate::{
    base::{
        paging_data::Paging, pg_client::PGClient,
        user_info::UserInfo,
    },
    data_models::notice::{NoticeComment, NoticeFriend, NoticePost, NoticeType},
    errors::MyError,
};

/// 发送通知
pub async fn send_notice(
    sender: &i32,
    notice_type: &NoticeType,
    sender_object: &String,
    addressee_id: &i32,
    client: &PGClient,
    redis_addr: &Addr<RedisActor>,
) -> Result<(), MyError> {
    let _stmt = include_str!("../../../sql/msg/insert_notices.sql");
    let stmt = client.prepare(_stmt).await?;

    let result = client
        .query(
            &stmt,
            &[sender, notice_type.to_i16(), sender_object, addressee_id],
        )
        .await?
        .iter()
        .map(|_| ())
        .collect::<Vec<()>>()
        .pop()
        .ok_or(MyError::InternalServerError);

    if let Err(ref e) = result {
        info!("send notice error: {}", e);
    } else {
        // 增加一个未读
        notice_type.incr(redis_addr, addressee_id);
    }
    result
}

/// 获取评论通知
pub async fn get_comment_notices<'a>(
    user: &UserInfo,
    paging: &Paging<'a>,
    client: &PGClient,
    redis_addr: &Addr<RedisActor>,
) -> Result<Vec<NoticeComment>, MyError> {
    let query = include_str!("../../../sql/msg/get_comment_notices.sql");
    let notice_type = NoticeType::Comment;
    let result = client.query_generics(query, &[
        notice_type.to_i16(),
        &user.id,
        paging.limit(),
        paging.offset(),
    ]).await;

    if result.is_ok() {
        // 清空评论通知
        notice_type.del(redis_addr, &user.id);
    }

    result
}

/// 获取点赞/反感通知
pub async fn get_post_notices<'a>(
    notice_type: &NoticeType,
    user: &UserInfo,
    paging: &Paging<'a>,
    client: &PGClient,
    redis_addr: &Addr<RedisActor>,
) -> Result<Vec<NoticePost>, MyError> {
    let query = include_str!("../../../sql/msg/get_post_notices.sql");
    let result = client.query_generics(query, &[
        notice_type.to_i16(),
        &user.id,
        paging.limit(),
        paging.offset(),
    ]).await;
    if result.is_ok() {
        // 清空点赞/反感通知
        notice_type.del(redis_addr, &user.id);
    }
    result
}

/// 获取好友通知
pub async fn get_friend_notices<'a>(
    notice_type: &NoticeType,
    user: &UserInfo,
    paging: &Paging<'a>,
    client: &PGClient,
    redis_addr: &Addr<RedisActor>,
) -> Result<Vec<NoticeFriend>, MyError> {
    let query = include_str!("../../../sql/msg/get_friend_notices.sql");
    let result = client.query_generics(query, &[
        notice_type.to_i16(),
        &user.id,
        paging.limit(),
        paging.offset(),
    ]).await;
    if result.is_ok() {
        // 清空好友通知
        notice_type.del(redis_addr, &user.id);
    }
    result
}