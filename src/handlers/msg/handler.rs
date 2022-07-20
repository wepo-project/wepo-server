use actix_web::{HttpResponse, web};

use crate::{base::{paging_data::{GetPageDTO, Paging}, pg_client::PGClient, user_info::UserInfo}, db};

pub async fn get_comments(
    user: UserInfo,
    body: web::Json<GetPageDTO>,
    client: PGClient,
) -> Result<HttpResponse, actix_web::Error> {
    let paging = Paging::default(&body.page)?;
    let list = db::msg::get_comments(&user, &client, &paging).await?;
    paging.finish(list)
}