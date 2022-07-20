use crate::{
    base::{user_info::UserInfo, pg_client::PGClient, paging_data::Paging}, db, errors::MyError,
    handlers::user::auth as AuthHandler, handlers::user::dto::*,
};

use actix_web::{web, Error, HttpResponse};
use log::info;

/// 用户注册
pub async fn register(
    user_info: web::Json<RegisterUserDTO>,
    client: PGClient,
) -> Result<HttpResponse, MyError> {
    if user_info.nick.is_empty() {
        return Err(MyError::err_code(301));
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
    client: PGClient,
    // redis: web::Data<Addr<RedisActor>>,
) -> Result<HttpResponse, Error> {
    let user_info: LoginUserDTO = user.into_inner();

    let user = db::user::validate_user(&client, user_info, false).await?;

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
    client: PGClient,
) -> Result<HttpResponse, MyError> {
    let user = db::user::validate_user(
        &client,
        LoginUserDTO {
            nick: user.nick.clone(),
            pwd: None,
        },
        true,
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
    client: PGClient,
) -> Result<HttpResponse, MyError> {
    let mut data = data.into_inner();
    let nick = db::user::change_nick(&client, &user.id, &data.nick).await?;
    data.nick = nick;
    Ok(HttpResponse::Ok().json(data))
}

/// 搜索用户
pub async fn search_user(
    body: web::Json<SearchUserDTO>,
    client: PGClient,
) -> Result<HttpResponse, actix_web::Error> {
    let paging = Paging::default(&body.page)?;
    let list = db::user::search_user(&client, &body.nick, &paging).await?;
    paging.finish(list)
}
