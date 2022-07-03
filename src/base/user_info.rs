use actix::fut;
use actix_web::{FromRequest, HttpMessage};

#[derive(Debug, Clone)]
pub struct UserInfo {
  pub id: i32,
  pub nick: String,
}

impl FromRequest for UserInfo {
    type Error = actix_web::Error;
    type Future = futures::future::Ready<Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        match req.extensions().get::<UserInfo>() {
            Some(user) => fut::ok(user.clone()),
            None => fut::err(actix_web::error::ErrorBadRequest("err"))
        }
    }
}