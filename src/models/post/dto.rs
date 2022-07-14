use serde::{Deserialize, Serialize};

use crate::data_models::PostExtends;

#[derive(Deserialize, Serialize)]
pub struct AddPostDTO {
    pub content: String,
}

#[derive(Deserialize, Serialize)]
pub struct AddPostResultDTO {
    pub id: String,
}

#[derive(Deserialize, Serialize)]
pub struct DelPostDTO {
    pub id: i64,
}


#[derive(Deserialize, Serialize)]
pub struct LikePostDTO {
    pub id: i64,
}


#[derive(Deserialize, Serialize)]
pub struct GetPostDTO {
    pub id: i64,
}

#[derive(Deserialize, Serialize)]
pub struct GetMyPostsDTO {
    pub page: i64,
}


#[derive(Deserialize, Serialize)]
pub struct GetMyPostsResultDTO {
    pub page: i64,
    pub next: bool,
    pub list: Vec<PostExtends>,
}


#[derive(Deserialize, Serialize)]
pub struct CommentPostDTO {
    pub content: String,
    pub origin_id: String,
}