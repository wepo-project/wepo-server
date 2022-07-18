use actix_web::{HttpResponse, web};

use crate::{errors::MyError, base::{paging_data::GetPageDTO, pg_client::PGClient}};

pub async fn get_notices(
    body: web::Json<GetPageDTO>,
    client: PGClient,
) -> Result<HttpResponse, MyError> {
    
    Ok(HttpResponse::Ok().finish())
}