use crate::{db, errors::MyError, dto::user::{RegisterUserDTO, LoginUserDTO, RegisterResultDTO, LoginResultDTO}};
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

    let result = RegisterResultDTO {
        id: new_user.id,
        nick: new_user.nick,
    };

    Ok(HttpResponse::Ok().json(result))
}

#[post("/login")]
/// 用户注册
pub async fn user_login(
    user: web::Json<LoginUserDTO>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let user_info: LoginUserDTO = user.into_inner();

    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let new_user = db::user::validate_user(&client, user_info).await?;

    let result = LoginResultDTO {
        id: new_user.id,
        nick: new_user.nick,
    };

    Ok(HttpResponse::Ok().json(result))
}