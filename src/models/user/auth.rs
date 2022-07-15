use crate::{
    base::user_info::UserInfo,
    errors::MyError,
};

use actix_web::{dev::ServiceRequest, Error, HttpMessage};
use chrono::Utc;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation};
use log::info;
use serde::{Deserialize, Serialize};

const JWT_SECRET: &[u8] = b"wepo_Jwt_Xecret";

pub fn create_jwt(id: &i32, _nick: &String) -> Result<String, MyError> {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(3600))
        .expect("valid timestamp")
        .timestamp();

    let header = Header::new(Algorithm::HS512);
    let claims = Claims::new(id, _nick, expiration as usize);

    jsonwebtoken::encode(&header, &claims, &EncodingKey::from_secret(JWT_SECRET))
        .map(|s| add_token_prefix(s))
        .map_err(|_| MyError::JWTTokenCreationError)
}

/// 把token加上前缀
pub fn add_token_prefix(token: String) -> String {
    format!("Bearer {}", token)
}

/// 验证token方法
pub fn validate_token(token: &str) -> Result<TokenData<Claims>, MyError> {
    let validation = Validation::new(Algorithm::HS512);
    let key = DecodingKey::from_secret(JWT_SECRET);
    let data = jsonwebtoken::decode::<Claims>(token, &key, &validation) //
        .map_err(|_e| {
            info!("token错误");
            MyError::JWTTokenError
        })?;
    Ok(data)
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