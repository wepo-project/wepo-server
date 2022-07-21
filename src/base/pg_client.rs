use std::{pin::Pin, ops::Deref};

use actix_web::{FromRequest, web};
use deadpool_postgres::{Client, Pool};
use futures::Future;
use tokio_postgres::Statement;

use crate::errors::MyError;

pub struct PGClient(Client);

impl PGClient {
    pub async fn prepare(&self, query: &str) -> Result<Statement, MyError> {
        self.0.prepare(query).await.map_err(MyError::PGError)
    }
}

impl FromRequest for PGClient {
    type Error = MyError;

    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &actix_web::HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        let db_pool = match req.app_data::<web::Data<Pool>>() {
            Some(val) => val.clone(),
            None => return Box::pin(async { Err(MyError::InternalServerError)})
        };
        Box::pin(async move {
            let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
            Ok(PGClient(client))
        })
    }
}

/// 智能指针
impl Deref for PGClient {
    type Target = Client;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}