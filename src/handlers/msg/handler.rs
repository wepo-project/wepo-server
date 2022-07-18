use actix_web::{HttpResponse, web};

use crate::{errors::MyError, base::{paging_data::{GetPageDTO, Paging}, pg_client::PGClient, user_info::UserInfo}, db};

pub async fn get_notices(
    user: UserInfo,
    body: web::Json<GetPageDTO>,
    client: PGClient,
) -> Result<HttpResponse, MyError> {
    let paging = Paging::default(&body.page);
    let list = db::msg::get_notices(&user, &client, &paging).await?;
    paging.finish(list)
}