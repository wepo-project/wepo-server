// use actix_web::http::header::Date;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Debug, Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "users")] // singular 'user' is a keyword..
pub struct User {
    pub id: i32,
    pub nick: String,
    pub pwd: Option<String>,
    pub _salt: String,
    pub create_time: NaiveDate,
}

#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "posts")] // singular 'user' is a keyword..
pub struct Post {
    pub id: String,
    pub sender: i32,
    pub content: String,
    pub create_time: i64,
    pub likes: i32,
}