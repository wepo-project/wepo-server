use actix::{Addr, spawn};
use actix_redis::RedisActor;
use actix_web::{web, Error, HttpResponse, Responder};
use log::info;

use crate::{
    base::{
        paging_data::{GetPageDTO, Paging},
        pg_client::PGClient,
        resp::ResultResponse,
        user_info::UserInfo,
    },
    data_models::notice::NoticeType,
    errors::MyError,
    handlers::MsgService,
    handlers::PostDTO::*,
};

use super::storage;

use super::dto::DelPostDTO;

pub async fn add(
    user: UserInfo,
    post_body: web::Json<AddPostDTO>,
    client: PGClient,
    redis_addr: web::Data<Addr<RedisActor>>,
) -> Result<HttpResponse, MyError> {
    let post_id = storage::add(&user, &post_body, &client, &redis_addr).await?;
    info!("New Post:{}", post_id);
    let result = AddPostResultDTO { id: post_id };
    Ok(HttpResponse::Ok().json(result))
}

/// 删除po
pub async fn delete(
    user: UserInfo,
    del_body: web::Json<DelPostDTO>,
    client: PGClient,
    redis_addr: web::Data<Addr<RedisActor>>,
) -> Result<HttpResponse, MyError> {
    let _ = storage::delete(&user, &del_body, &client, &redis_addr).await?;
    Ok(HttpResponse::Ok().json(ResultResponse::succ()))
}

/// 获取po
pub async fn get_one(
    user: UserInfo,
    body: web::Query<GetPostDTO>,
    client: PGClient,
    redis_addr: web::Data<Addr<RedisActor>>,
) -> Result<HttpResponse, MyError> {
    let post = storage::get_one(&user, &body.id, &client, &redis_addr).await?;
    Ok(HttpResponse::Ok().json(post))
}

/// 点赞
pub async fn like(
    user: UserInfo,
    data: web::Query<LikePostDTO>,
    redis_addr: web::Data<Addr<RedisActor>>,
    client: PGClient,
) -> Result<HttpResponse, Error> {
    let _ = storage::like(&data.id, &user.id, &redis_addr).await?;
    spawn(async move {
        let _ = MsgService::sender_post_notice(
            &NoticeType::Like,
            &user.id,
            &data.id,
            &client,
            &redis_addr,
        ).await;
    });
    Ok(HttpResponse::Ok().json(ResultResponse::succ()))
}

/// 取消点赞
pub async fn cancel_like(
    user: UserInfo,
    like_body: web::Query<LikePostDTO>,
    redis_addr: web::Data<Addr<RedisActor>>,
) -> Result<HttpResponse, Error> {
    let _ = storage::cancel_like(&like_body.id, &user.id, &redis_addr).await?;
    Ok(HttpResponse::Ok().json(ResultResponse::succ()))
}

/// 获取我的posts
pub async fn mine(
    user: UserInfo,
    body: web::Json<GetPageDTO>,
    client: PGClient,
    redis_addr: web::Data<Addr<RedisActor>>,
) -> Result<HttpResponse, actix_web::Error> {
    let paging = Paging::default(&body.page)?;
    let list = storage::get_mine(&user, &paging, &client, &redis_addr).await?;
    paging.finish(list)
}

/// 评论
pub async fn comment(
    user: UserInfo,
    body: web::Json<CommentPostDTO>,
    client: PGClient,
    redis_addr: web::Data<Addr<RedisActor>>,
) -> Result<HttpResponse, MyError> {
    let comment_result = storage::comment(&user, &body, &client).await?;
    info!("New Comment:{}", comment_result.id);
    // 评论成功，发送通知, 如果评论自己就不发送了
    if user.id != comment_result.receiver {
        MsgService::send_comment_notice(
            &user.id,
            &comment_result.receiver,
            &comment_result.id,
            &client,
            &redis_addr
        )
        .await;
    }

    let result = AddPostResultDTO {
        id: comment_result.id,
    };
    Ok(HttpResponse::Ok().json(result))
}

/// 反感
pub async fn hate(
    user: UserInfo,
    data: web::Query<LikePostDTO>,
    client: PGClient,
    redis_addr: web::Data<Addr<RedisActor>>,
) -> Result<HttpResponse, Error> {
    let _ = storage::hate(&data.id, &user.id, &redis_addr).await?;
    spawn(async move {
        let _ = MsgService::sender_post_notice(
            &NoticeType::Hate,
            &user.id,
            &data.id,
            &client,
            &redis_addr,
        ).await;
    });
    Ok(HttpResponse::Ok().json(ResultResponse::succ()))
}

/// 取消反感
pub async fn cancel_hate(
    user: UserInfo,
    like_body: web::Query<LikePostDTO>,
    redis_addr: web::Data<Addr<RedisActor>>,
) -> Result<HttpResponse, Error> {
    let _ = storage::cancel_hate(&like_body.id, &user.id, &redis_addr).await?;
    Ok(HttpResponse::Ok().json(ResultResponse::succ()))
}

/// 浏览posts
pub async fn browse(
    user: UserInfo,
    body: web::Query<GetPageDTO>,
    client: PGClient,
    redis_addr: web::Data<Addr<RedisActor>>,
) -> Result<impl Responder, actix_web::Error> {
    let paging = Paging::default(&body.page)?;
    let list = storage::browse(&user, &client, &paging, &redis_addr).await?;
    paging.finish(list)
}
