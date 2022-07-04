use serde::{Deserialize, Serialize};
use uuid::Uuid;

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