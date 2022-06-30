use crate::{db, errors::MyError, dto::user::RegisterUserDTO};
use actix_web::{web, Error, HttpResponse, post};
use deadpool_postgres::{Client, Pool};

#[post("/add_user")]
/// 用户注册
pub async fn register_user(
    user: web::Json<RegisterUserDTO>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let user_info: RegisterUserDTO = user.into_inner();

    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let new_user = db::user::add_user(&client, user_info).await?;

    Ok(HttpResponse::Ok().json(new_user))
}