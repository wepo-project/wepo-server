use actix::Addr;
use actix_redis::RedisActor;
use actix_web::{web, Error, HttpResponse, Responder};
use deadpool_postgres::{Client, Pool};
use log::info;

use crate::{
    base::{
        paging_data::{PagingData, PagingDataBuilder, GetPageDTO},
        resp::ResultResponse,
        user_info::UserInfo, pg_client::PGClient,
    },
    db,
    errors::MyError,
    handlers::post::dto::*,
};

use super::dto::DelPostDTO;

pub async fn add(
    user: UserInfo,
    post_body: web::Json<AddPostDTO>,
    client: PGClient,
) -> Result<HttpResponse, MyError> {
    let post_id = db::post::add(&user, &post_body, &client).await?;
    info!("New Post:{}", post_id);
    let result = AddPostResultDTO {
        id: post_id.to_string(),
    };
    Ok(HttpResponse::Ok().json(result))
}

/// 删除po
pub async fn delete(
    user: UserInfo,
    del_body: web::Json<DelPostDTO>,
    client: PGClient,
    redis_addr: web::Data<Addr<RedisActor>>,
) -> Result<HttpResponse, MyError> {
    let _ = db::post::delete(&user, &del_body, &client, &redis_addr).await?;
    Ok(HttpResponse::Ok().json(ResultResponse::succ()))
}

/// 获取po
pub async fn get_one(
    user: UserInfo,
    body: web::Query<GetPostDTO>,
    client: PGClient,
    redis_addr: web::Data<Addr<RedisActor>>,
) -> Result<HttpResponse, MyError> {
    let post = db::post::get_one(&user, &body.id, &client, &redis_addr).await?;
    Ok(HttpResponse::Ok().json(post))
}

/// 点赞
pub async fn like(
    user: UserInfo,
    like_body: web::Query<LikePostDTO>,
    redis_addr: web::Data<Addr<RedisActor>>,
) -> Result<HttpResponse, Error> {
    let _ = db::post::like(&like_body.id, &user.id, &redis_addr).await?;
    Ok(HttpResponse::Ok().json(ResultResponse::succ()))
}

/// 取消点赞
pub async fn cancel_like(
    user: UserInfo,
    like_body: web::Query<LikePostDTO>,
    redis_addr: web::Data<Addr<RedisActor>>,
) -> Result<HttpResponse, Error> {
    let _ = db::post::cancel_like(&like_body.id, &user.id, &redis_addr).await?;
    Ok(HttpResponse::Ok().json(ResultResponse::succ()))
}

/// 获取我的posts
pub async fn mine(
    user: UserInfo,
    body: web::Json<GetPageDTO>,
    client: PGClient,
    redis_addr: web::Data<Addr<RedisActor>>,
) -> Result<HttpResponse, Error> {
    /// 每页的数量
    const COUNT_PER_PAGE: i64 = 20;

    Ok(HttpResponse::Ok().json(
        PagingDataBuilder::new(&COUNT_PER_PAGE, &body.page)
        .set_list(db::post::get_mine(&user, &body.page, &COUNT_PER_PAGE, &client, &redis_addr).await?)
    ))
}

/// 评论
pub async fn comment(
    user: UserInfo,
    body: web::Json<CommentPostDTO>,
    client: PGClient,
) -> Result<HttpResponse, MyError> {
    let post_id = db::post::comment(&user, &body, &client).await?;
    info!("New Comment:{}", post_id);
    let result = AddPostResultDTO {
        id: post_id.to_string(),
    };
    Ok(HttpResponse::Ok().json(result))
}

/// 反感
pub async fn hate(
    user: UserInfo,
    like_body: web::Query<LikePostDTO>,
    redis_addr: web::Data<Addr<RedisActor>>,
) -> Result<HttpResponse, Error> {
    let _ = db::post::hate(&like_body.id, &user.id, &redis_addr).await?;
    Ok(HttpResponse::Ok().json(ResultResponse::succ()))
}

/// 取消反感
pub async fn cancel_hate(
    user: UserInfo,
    like_body: web::Query<LikePostDTO>,
    redis_addr: web::Data<Addr<RedisActor>>,
) -> Result<HttpResponse, Error> {
    let _ = db::post::cancel_hate(&like_body.id, &user.id, &redis_addr).await?;
    Ok(HttpResponse::Ok().json(ResultResponse::succ()))
}

/// 浏览posts
pub async fn browse(
    user: UserInfo,
    body: web::Query<GetPageDTO>,
    client: PGClient,
    redis_addr: web::Data<Addr<RedisActor>>,
) -> Result<impl Responder, Error> {
    /// 每页的数量
    const COUNT_PER_PAGE: i64 = 20;
    Ok(HttpResponse::Ok().json(
        PagingDataBuilder::new(&COUNT_PER_PAGE, &body.page)
        .set_list(db::post::browse(&user, &client, &body.page, &COUNT_PER_PAGE, &redis_addr).await?)
    ))
}
