
use actix::Addr;
use actix_redis::RedisActor;
use actix_web::{delete, get, post, web, Error, HttpResponse};
use deadpool_postgres::{Client, Pool};
use log::info;

use crate::{
    base::{resp::ResultResponse, user_info::UserInfo},
    db,
    errors::MyError,
    models::post::dto::*,
};

use super::dto::DelPostDTO;

#[post("/add_post")]
pub async fn add_post(
    user: UserInfo,
    post_body: web::Json<AddPostDTO>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, MyError> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let post = db::post::add_post(&user, &post_body, &client).await?;
    info!("New Post:{}", post.id);
    let result = AddPostResultDTO { id: post.id };
    Ok(HttpResponse::Ok().json(result))
}

/// 删除po
#[delete("/del_post")]
pub async fn delete_post(
    user: UserInfo,
    del_body: web::Json<DelPostDTO>,
    db_pool: web::Data<Pool>,
    redis_addr: web::Data<Addr<RedisActor>>,
) -> Result<HttpResponse, MyError> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let _ = db::post::del_post(&user, &del_body, &client, &redis_addr).await?;
    Ok(HttpResponse::Ok().json(ResultResponse::succ()))
}

/// 获取po
#[get("/get_post")]
pub async fn get_post(
    body: web::Json<GetPostDTO>,
    db_pool: web::Data<Pool>,
    redis_addr: web::Data<Addr<RedisActor>>,
) -> Result<HttpResponse, MyError> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let post = db::post::get_post(&body.id, &client, &redis_addr).await?;
    Ok(HttpResponse::Ok().json(post))
}

/// 点赞
#[get("/like")]
pub async fn post_like(
    user: UserInfo,
    like_body: web::Json<LikePostDTO>,
    redis_addr: web::Data<Addr<RedisActor>>,
) -> Result<HttpResponse, Error> {
    let _ = db::post::like_post(&like_body.id, &user.id, &redis_addr).await?;
    Ok(HttpResponse::Ok().json(ResultResponse::succ()))
}

/// 取消点赞
#[get("/unlike")]
pub async fn post_unlike(
    user: UserInfo,
    like_body: web::Json<LikePostDTO>,
    redis_addr: web::Data<Addr<RedisActor>>,
) -> Result<HttpResponse, Error> {
    let _ = db::post::unlike_post(&like_body.id, &user.id, &redis_addr).await?;
    Ok(HttpResponse::Ok().json(ResultResponse::succ()))
}