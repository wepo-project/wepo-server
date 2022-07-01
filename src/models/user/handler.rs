use crate::{db, errors::MyError, models::user::dto::{RegisterUserDTO, LoginUserDTO, RegisterResultDTO, LoginResultDTO}, utils};
use actix::Addr;
use actix_redis::{RedisActor, Command, resp_array};
use actix_web::{web, Error, HttpResponse, post};
use deadpool_postgres::{Client, Pool};
use log::info;

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
    redis: web::Data<Addr<RedisActor>>
) -> Result<HttpResponse, Error> {
    let user_info: LoginUserDTO = user.into_inner();

    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let user = db::user::validate_user(&client, user_info).await?;

    let token = utils::get_random_string(12);

    let result = LoginResultDTO {
        id: user.id,
        nick: user.nick,
        token: token.clone(),
    };

    info!("User Login:{:?}", result);

    // 设置token到redis上
    let _res = redis.send(Command(resp_array![
        "SET", format!("player:{}", user.id), token
    ])).await.map_err(MyError::MailboxError);

    Ok(HttpResponse::Ok().json(result))
}