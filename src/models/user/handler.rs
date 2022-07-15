use crate::{
    base::user_info::UserInfo,
    models::user::auth as AuthHandler,
    db,
    errors::MyError,
    models::user::dto::*,
};

use actix_web::{web, Error, HttpResponse};
use deadpool_postgres::{Client, Pool};
use log::info;

/// 用户注册
pub async fn register(
    user_info: web::Json<RegisterUserDTO>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, MyError> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    if user_info.nick.is_empty() {
        return Err(MyError::code(301));
    }
    let new_user = db::user::add(&client, user_info.0).await?;
    info!("creating a new user:{}", new_user.nick);
    let result = RegisterResultDTO {
        id: new_user.id,
        nick: new_user.nick,
    };
    Ok(HttpResponse::Ok().json(result))
}

/// 用户登录
pub async fn login(
    user: web::Json<LoginUserDTO>,
    db_pool: web::Data<Pool>,
    // redis: web::Data<Addr<RedisActor>>,
) -> Result<HttpResponse, Error> {
    let user_info: LoginUserDTO = user.into_inner();

    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let user = db::user::validate_user(&client, user_info).await?;

    let token = AuthHandler::create_jwt(&user.id, &user.nick)?;

    info!("User Login:{:?}", user);

    let result = LoginResultDTO {
        id: user.id,
        nick: user.nick,
        token: token.clone(),
    };

    Ok(HttpResponse::Ok().json(result))
}

/// 用token登录
pub async fn login_with_token(user: UserInfo) -> Result<HttpResponse, MyError> {
    info!("User Login:{:?}", user);
    let new_token = AuthHandler::create_jwt(&user.id, &user.nick)?;
    Ok(HttpResponse::Ok().json(new_token))
}

/// 修改昵称
pub async fn change_nick(
    user: UserInfo,
    data: web::Json<ChangeNickDTO>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, MyError> {
    let mut data = data.into_inner();
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let nick = db::user::change_nick(&client, &user.id, &data.nick).await?;
    data.nick = nick;
    Ok(HttpResponse::Ok().json(data))
}

// ============================================
