use actix::{spawn, Addr};
use actix_redis::RedisActor;
use actix_web::{web, HttpResponse};

use crate::{
    base::{pg_client::PGClient, resp::ResultResponse, user_info::UserInfo},
    data_models::notice::NoticeType,
    errors::MyError,
    handlers::MsgService,
};

use super::{dto::SendFriendRequestDTO, storage};

/// 添加好友 （单向）
pub async fn add_friendship(
    user: UserInfo,
    client: PGClient,
    body: web::Json<SendFriendRequestDTO>,
    redis_addr: web::Data<Addr<RedisActor>>,
) -> Result<HttpResponse, MyError> {
    let _ = storage::add_friend(&user, &body.user_id, &client).await?;
    spawn(async move {
        // 通知
        MsgService::send_friend_notice(
            &NoticeType::FriendAdd,
            &user.id,
            &body.user_id,
            &body.msg,
            &client,
            &redis_addr,
        )
        .await;
    });
    Ok(HttpResponse::Ok().json(ResultResponse::succ()))
}

/// 移除好友 （单向）
pub async fn remove_friendship(
    user: UserInfo,
    client: PGClient,
    body: web::Json<SendFriendRequestDTO>,
    redis_addr: web::Data<Addr<RedisActor>>,
) -> Result<HttpResponse, MyError> {
    let _ = storage::remove_friend(&user, &body.user_id, &client).await?;
    spawn(async move {
        // 通知
        MsgService::send_friend_notice(
            &NoticeType::FriendRemove,
            &user.id,
            &body.user_id,
            &body.msg,
            &client,
            &redis_addr
        )
        .await;
    });
    Ok(HttpResponse::Ok().json(ResultResponse::succ()))
}
