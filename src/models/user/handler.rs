use crate::{base::user_info::UserInfo, db, errors::MyError, models::user::dto::*};

use actix_web::{dev::ServiceRequest, post, web, Error, HttpMessage, HttpResponse};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use chrono::Utc;
use deadpool_postgres::{Client, Pool};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use log::info;
use serde::{Deserialize, Serialize};

#[post("/add_user")]
/// 用户注册
pub async fn register_user(
    user: web::Json<RegisterUserDTO>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let user_info: RegisterUserDTO = user.into_inner();
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let new_user = db::user::add_user(&client, user_info).await?;
    info!("creating a new user:{}", new_user.nick);
    let result = RegisterResultDTO {
        id: new_user.id,
        nick: new_user.nick,
    };
    Ok(HttpResponse::Ok().json(result))
}

#[post("/login")]
/// 用户登录
pub async fn user_login(
    user: web::Json<LoginUserDTO>,
    db_pool: web::Data<Pool>,
    // redis: web::Data<Addr<RedisActor>>,
) -> Result<HttpResponse, Error> {
    let user_info: LoginUserDTO = user.into_inner();

    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let user = db::user::validate_user(&client, user_info).await?;

    let token = format!("Bearer {}", create_jwt(&user.id, &user.nick)?);

    let result = LoginResultDTO {
        id: user.id,
        nick: user.nick,
        token: token.clone(),
    };

    info!("User Login:{:?}", result);

    Ok(HttpResponse::Ok().json(result))
}

const JWT_SECRET: &[u8] = b"wepo_Jwt_Xecret";

pub fn create_jwt(id: &i32, _nick: &String) -> Result<String, MyError> {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(3600))
        .expect("valid timestamp")
        .timestamp();

    let header = Header::new(Algorithm::HS512);
    let claims = Claims::new(id, _nick, expiration as usize);

    jsonwebtoken::encode(&header, &claims, &EncodingKey::from_secret(JWT_SECRET))
        .map_err(|_| MyError::JWTTokenCreationError)
}

pub async fn bearer_handle(req: ServiceRequest, auth: BearerAuth) -> Result<ServiceRequest, Error> {
    let token = auth.token();
    let validation = Validation::new(Algorithm::HS512);
    let key = DecodingKey::from_secret(JWT_SECRET);
    let decoded = jsonwebtoken::decode::<Claims>(token, &key, &validation).map_err(|_| {
        info!("token错误");
        MyError::JWTTokenError
    })?;

    // info!("{:?}", decoded);

    // 添加进拓展值，后续的handler直接在参数中可以直接使用
    req.extensions_mut().insert(decoded.claims.into_user_info());

    Ok(req)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    iss: String,
    pub sub: String,
    pub exp: usize,
    pub id: i32,
}

impl Claims {
    pub(crate) fn new(id: &i32, nick: &String, exp: usize) -> Self {
        Self {
            iss: String::from("wepo"),
            sub: nick.to_owned(),
            id: *id,
            exp,
        }
    }

    pub fn into_user_info(self) -> UserInfo {
        UserInfo {
            id: self.id,
            nick: self.sub,
        }
    }
}

// #[derive(Debug, Serialize, Deserialize)]
// struct Claims {
//     aud: String,         // Optional. Audience
//     exp: usize,          // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
//     iat: usize,          // Optional. Issued at (as UTC timestamp)
//     iss: String,         // Optional. Issuer
//     nbf: usize,          // Optional. Not Before (as UTC timestamp)
//     sub: String,         // Optional. Subject (whom token refers to)
// }
