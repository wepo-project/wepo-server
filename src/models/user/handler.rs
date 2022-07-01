
use crate::{db, errors::MyError, models::user::dto::{RegisterUserDTO, LoginUserDTO, RegisterResultDTO, LoginResultDTO}, utils};
use actix::Addr;
use actix_redis::{RedisActor, Command, resp_array};
use actix_web::{web, Error, HttpResponse, post, guard::Guard, dev::ServiceRequest};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use chrono::Utc;
use deadpool_postgres::{Client, Pool};
use jsonwebtoken::{Header, Algorithm, EncodingKey, decode, DecodingKey, Validation};
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

pub fn create_jwt(id: &i32, nick: &String) -> Result<String, MyError> {
    let expiration = Utc::now().checked_add_signed(chrono::Duration::seconds(60))
    .expect("valid timestamp")
    .timestamp();
    
    let header = Header::new(Algorithm::HS512);
    let claims = Claims {
        sub: id.to_string(),
        exp: expiration as usize,
    };

    jsonwebtoken::encode(&header, &claims, &EncodingKey::from_secret(JWT_SECRET)).map_err(|_| MyError::JWTTokenCreationError)
}

pub async fn bearer_handle(req: ServiceRequest, auth: BearerAuth) -> Result<ServiceRequest, Error> {
    let token = auth.token();
    let decoded = jsonwebtoken::decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET),
        &Validation::new(Algorithm::HS512),
    ).map_err(|_| MyError::JWTTokenError)?;
    Ok(req)
} 

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Claims {
    pub sub: String,
    pub exp: usize,
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