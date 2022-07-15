
use actix::MailboxError;
use actix_redis::Error as RedisError;
use actix_web::{HttpResponse, ResponseError};
use deadpool_postgres::PoolError;
use derive_more::{Display, From};
use log::info;
use serde::{Deserialize, Serialize};
use tokio_pg_mapper::Error as PGMError;
use tokio_postgres::error::Error as PGError;

use crate::base::resp::ResultResponse;

#[derive(Display, From, Debug)]
pub enum MyError {
    NotFound,
    JWTTokenCreationError,
    FailResultError,
    JWTTokenError,
    AuthorizationNotFound,
    ParseError,
    /// 互斥锁中毒错误
    PoisonError,
    PGError(PGError),
    PGMError(PGMError),
    PoolError(PoolError),
    MailboxError(MailboxError),
    RedisError(RedisError),
    OkError(i32),
}

impl MyError {
    pub fn code(code: i32) -> Self {
        MyError::OkError(code)
    }
}

impl std::error::Error for MyError {}

impl ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        info!("{}", self);
        match *self {
            MyError::NotFound => HttpResponse::NotFound().finish(),
            MyError::JWTTokenError => HttpResponse::Unauthorized().finish(),
            MyError::PoolError(ref err) => {
                HttpResponse::InternalServerError().body(err.to_string())
            }
            MyError::OkError(ref code) => HttpResponse::Ok().json(ErrorResponse::new(code)),
            MyError::FailResultError => HttpResponse::Ok().json(ResultResponse::fail()),
            _ => HttpResponse::InternalServerError().finish(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub code: i32,
}

impl ErrorResponse {
    pub fn new(code: &i32) -> Self {
        Self { code: *code }
    }
}
