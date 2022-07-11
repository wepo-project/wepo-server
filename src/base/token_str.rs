use actix::fut;
use actix_web::{FromRequest, HttpMessage};

#[derive(Debug, Clone)]
pub struct TokenStr {
  pub token: String,
}

impl FromRequest for TokenStr {
    type Error = actix_web::Error;
    type Future = futures::future::Ready<Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        match req.extensions().get::<TokenStr>() {
            Some(td) => fut::ok(td.clone()),
            None => fut::err(actix_web::error::ErrorBadRequest("err"))
        }
    }
}