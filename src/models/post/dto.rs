use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct AddPostDTO {
    pub content: String,
}