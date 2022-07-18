use actix::fut::{Ready, ready};
use actix_web::{FromRequest, web};
use deadpool_postgres::{Client, Pool};

use crate::errors::MyError;

pub struct PGClient(Client);

// impl FromRequest for PGClient {
//     type Error = MyError;

//     type Future = Ready<Result<Self, Self::Error>>;

//     fn from_request(req: &actix_web::HttpRequest, payload: &mut actix_web::dev::Payload) -> Self::Future {
//         // ready(Box::pin(async {
//         //     if let Some(db_pool) = req.app_data::<web::Data<Pool>>() {
//         //         Ok(db_pool.get().await.map_err(MyError::PoolError)?)
//         //     }
//         // }.await))
//     }
// }