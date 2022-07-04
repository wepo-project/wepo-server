use actix_web::{HttpResponse, post, web, delete};
use deadpool_postgres::{Pool, Client};
use log::info;

use crate::{models::post::dto::*, errors::MyError, base::{user_info::UserInfo, resp::ResultResponse}, db::{self, post}};

use super::dto::DelPostDTO;

#[post("/add_post")]
pub async fn add_post(
    user: UserInfo,
    post_body: web::Json<AddPostDTO>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, MyError> {
    let post_data = post_body.into_inner();

    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let post = db::post::add_post(&user, &post_data, &client).await?;

    info!("New Post:{}", post.id);

    let result = AddPostResultDTO { id: post.id.hyphenated().to_string() };

    Ok(HttpResponse::Ok().json(result))
}

#[delete("/del_post")]
pub async fn delete_post(
    user: UserInfo,
    del_body: web::Json<DelPostDTO>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, MyError> {
    let body = del_body.into_inner();

    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let _ = db::post::del_post(&user, &body, &client).await?;

    Ok(HttpResponse::Ok().json(ResultResponse::succ()))
}
