use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::{data_models::Post, models::user::data::UserData};

#[derive(Debug, Deserialize, Serialize)]
pub struct PostData {
    pub id: String,
    pub sender: UserData,
    pub content: String,
    pub create_time: NaiveDateTime,
    pub likes: i64,
    pub comments: i64,
    pub reposts: i64,
    // #[serde(skip_serializing)]
    pub extends: Option<i64>,
    pub extends_info: Option<Box<PostData>>,
}

impl PostData {
    pub async fn new(post: &Post) -> Self {
        
        Self {
            id: post.id.to_string(),

        }
    }
}