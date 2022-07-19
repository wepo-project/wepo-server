use tokio_postgres::Row;

use crate::base::big_int::BigInt;

/// 评论的结果
pub struct CommentResult {
    pub id: BigInt,
    pub extends: BigInt,
    pub receiver: i32,
}

impl From<&Row> for CommentResult {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get("id"),
            extends: row.get("extends"),
            receiver: row.get("receiver"),
        }
    }
}