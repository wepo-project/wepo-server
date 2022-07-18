use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Debug, Deserialize, Serialize, PostgresMapper)]
#[pg_mapper(table = "notices")]
pub struct Notice {
    pub notice_id: String,
    pub addressee_id: i32,
    pub notice_type: String,
    pub args: Vec<String>,
    pub create_time: NaiveDateTime,
    pub read: bool,
}

