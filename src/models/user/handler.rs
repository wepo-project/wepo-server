use crate::{
    base::user_info::UserInfo, data_models::UserData, db, errors::MyError,
    models::user::auth as AuthHandler, models::user::dto::*,
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
        user,
        token: token.clone(),
    };

    Ok(HttpResponse::Ok().json(result))
}

/// 用token登录
pub async fn login_with_token(
    user: UserInfo,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, MyError> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let user = db::user::validate_user(
        &client,
        LoginUserDTO {
            nick: user.nick.clone(),
            pwd: None,
        },
    )
    .await?;
    let new_token = AuthHandler::create_jwt(&user.id, &user.nick)?;
    info!("User Login:{:?}", user);
    Ok(HttpResponse::Ok().json(LoginResultDTO {
        user,
        token: new_token,
    }))
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

/// 搜索用户
pub async fn search_user(
    data: web::Json<SearchUserDTO>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, MyError> {
    /// 每页的数量
    const COUNT_PER_PAGE: i64 = 20;
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let list = db::user::search_user(&client, &data.nick, &data.page, &COUNT_PER_PAGE).await?;
    let next = list.len() >= COUNT_PER_PAGE as usize;
    Ok(HttpResponse::Ok().json(SearchUserResultDTO {
        page: data.page,
        next,
        list,
    }))
}
