use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::data_models::Post;

#[derive(Deserialize, Serialize)]
pub struct AddPostDTO {
    pub content: String,
}

#[derive(Deserialize, Serialize)]
pub struct AddPostResultDTO {
    pub id: Uuid,
}

#[derive(Deserialize, Serialize)]
pub struct DelPostDTO {
    pub id: Uuid,
}


#[derive(Deserialize, Serialize)]
pub struct LikePostDTO {
    pub id: Uuid,
}


#[derive(Deserialize, Serialize)]
pub struct GetPostDTO {
    pub id: Uuid,
}

#[derive(Deserialize, Serialize)]
pub struct GetMyPostsDTO {
    pub page: i64,
}


#[derive(Deserialize, Serialize)]
pub struct GetMyPostsResultDTO {
    pub page: i64,
    pub next: bool,
    pub list: Vec<Post>,
}

