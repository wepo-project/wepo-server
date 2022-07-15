use actix::fut;
use actix_web::FromRequest;

use crate::{models::user::auth::validate_token, errors::MyError};

#[derive(Debug, Clone)]
pub struct UserInfo {
    pub id: i32,
    pub nick: String,
}

impl FromRequest for UserInfo {
    type Error = MyError;
    type Future = futures::future::Ready<Result<Self, Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        fut::ready({
            let auth = req.headers().get("Authorization");
            match auth {
                Some(val) => {
                    let token = val
                        .to_str()
                        .unwrap_or("")
                        .split("Bearer ")
                        .collect::<Vec<&str>>()
                        .pop()
                        .unwrap_or("");
                    let token_data = validate_token(token);
                    match token_data {
                        Ok(data) => Ok(data.claims.into_user_info()),
                        Err(e) => Err(e)
                    }
                }
                None => Err(MyError::AuthorizationNotFound),
            }
        })
    }
}