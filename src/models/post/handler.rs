use actix_web::{HttpResponse, post, web};

use crate::{models::post::dto::AddPostDTO, errors::MyError};

#[post("/add_post")]
pub async fn add_post(
    post_body: web::Json<AddPostDTO>
) -> Result<HttpResponse, MyError> {
    Err(MyError::code(200))
}