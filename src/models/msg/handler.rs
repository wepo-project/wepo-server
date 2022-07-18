use actix_web::{HttpResponse, web};

use crate::{errors::MyError, base::paging_data::GetPageDTO};

pub async fn get_notices(
    body: web::Json<GetPageDTO>,
) -> Result<HttpResponse, MyError> {
    Ok(HttpResponse::Ok().finish())
}