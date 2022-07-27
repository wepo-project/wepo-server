use actix::Addr;
use actix_redis::RedisActor;

use crate::{
    base::{big_int::BigInt, pg_client::PGClient},
    data_models::notice::NoticeType,
    errors::MyError,
    handlers::PostService,
};

use super::storage;

/// 发送评论通知
pub async fn send_comment_notice(
    sender_id: &i32,
    receiver_id: &i32,
    post_id: &BigInt,
    client: &PGClient,
    redis_addr: &Addr<RedisActor>,
) -> () {
    let _result = storage::send_notice(
        sender_id,
        &NoticeType::Comment,
        &post_id.to_string(),
        receiver_id,
        &client,
        redis_addr,
    )
    .await;
}

/// 发送 post 点赞/反感通知
pub async fn sender_post_notice(
    notice_type: &NoticeType,
    sender_id: &i32,
    post_id: &BigInt,
    client: &PGClient,
    redis_addr: &Addr<RedisActor>,
) -> Result<(), MyError> {
    let addressee_id = PostService::get_post_sender_from_id(post_id, client, redis_addr).await;
    // info!("收件人id:{:?}", addressee_id);

    if let Ok(addressee_id) = addressee_id {
        // 自己给自己点赞不通知
        if &addressee_id != sender_id {
            let _result = storage::send_notice(
                sender_id,
                &notice_type,
                &post_id.to_string(),
                &addressee_id,
                &client,
                redis_addr
            )
            .await;
        }
    }

    Ok(())
}

/// 发送好友添加通知
pub async fn send_friend_notice(
    notice_type: &NoticeType,
    sender_id: &i32,
    receiver_id: &i32,
    msg: &String,
    client: &PGClient,
    redis_addr: &Addr<RedisActor>,
) -> () {
    let _result = storage::send_notice(sender_id, &notice_type, &msg, receiver_id, &client, redis_addr).await;
}
