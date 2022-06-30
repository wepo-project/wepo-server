use actix_web::{HttpResponse, ResponseError};
use deadpool_postgres::PoolError;
use derive_more::{Display, From};
use tokio_pg_mapper::Error as PGMError;
use tokio_postgres::error::Error as PGError;
use serde::{Serialize, Deserialize};

#[derive(Display, From, Debug)]
pub enum MyError {
    NotFound,
    PGError(PGError),
    PGMError(PGMError),
    PoolError(PoolError),
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
        match *self {
            MyError::NotFound => HttpResponse::NotFound().finish(),
            MyError::PoolError(ref err) => {
                HttpResponse::InternalServerError().body(err.to_string())
            }
            MyError::OkError(ref code) => {
                HttpResponse::Ok().json(ErrorResponse::new(code))
            }
            _ => HttpResponse::InternalServerError().finish(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub code: i32
}

impl ErrorResponse {
    pub fn new(code: &i32) -> Self {
        Self { code: *code }
    }
}