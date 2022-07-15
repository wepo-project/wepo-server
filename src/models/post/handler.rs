
use actix::Addr;
use actix_redis::RedisActor;
use actix_web::{web, Error, HttpResponse};
use deadpool_postgres::{Client, Pool};
use log::info;

use crate::{
    base::{resp::ResultResponse, user_info::UserInfo},
    db,
    errors::MyError,
    models::post::dto::*,
};

use super::dto::DelPostDTO;

pub async fn add(
    user: UserInfo,
    post_body: web::Json<AddPostDTO>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, MyError> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let post_id = db::post::add(&user, &post_body, &client).await?;
    info!("New Post:{}", post_id);
    let result = AddPostResultDTO { id: post_id.to_string() };
    Ok(HttpResponse::Ok().json(result))
}

/// 删除po
pub async fn delete(
    user: UserInfo,
    del_body: web::Json<DelPostDTO>,
    db_pool: web::Data<Pool>,
    redis_addr: web::Data<Addr<RedisActor>>,
) -> Result<HttpResponse, MyError> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let _ = db::post::delete(&user, &del_body, &client, &redis_addr).await?;
    Ok(HttpResponse::Ok().json(ResultResponse::succ()))
}

/// 获取po
pub async fn get_one(
    user: UserInfo,
    body: web::Query<GetPostDTO>,
    db_pool: web::Data<Pool>,
    redis_addr: web::Data<Addr<RedisActor>>,
) -> Result<HttpResponse, MyError> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
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
    body: web::Json<GetMyPostsDTO>,
    db_pool: web::Data<Pool>,
    redis_addr: web::Data<Addr<RedisActor>>,
) -> Result<HttpResponse, Error> {
    /// 每页的数量
    const COUNT_PER_PAGE: i64 = 20;
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let post = db::post::get_mine(&user, &body.page, &COUNT_PER_PAGE, &client, &redis_addr).await?;
    let next = post.len() >= COUNT_PER_PAGE as usize;
    Ok(HttpResponse::Ok().json(GetMyPostsResultDTO{
        page: body.page,
        next,
        list: post,
    }))
}

/// 评论
pub async fn comment(
    user: UserInfo,
    body: web::Json<CommentPostDTO>,
    db_pool: web::Data<Pool>,
    redis_addr: web::Data<Addr<RedisActor>>,
) -> Result<HttpResponse, MyError> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let post_id = db::post::comment(&user, &body, &client, &redis_addr).await?;
    info!("New Comment:{}", post_id);
    let result = AddPostResultDTO { id: post_id.to_string() };
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