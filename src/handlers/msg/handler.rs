use actix_web::{HttpResponse, web};

use crate::{base::{paging_data::{GetPageDTO, Paging}, pg_client::PGClient, user_info::UserInfo}, db};

/// 获取评论通知
pub async fn get_comment_notices(
    user: UserInfo,
    body: web::Json<GetPageDTO>,
    client: PGClient,
) -> Result<HttpResponse, actix_web::Error> {
    let paging = Paging::default(&body.page)?;
    let list = db::msg::get_comment_notices(&user, &client, &paging).await?;
    paging.finish(list)
}

/// 获取评论通知
pub async fn get_like_notices(
    user: UserInfo,
    body: web::Json<GetPageDTO>,
    client: PGClient,
) -> Result<HttpResponse, actix_web::Error> {
    let paging = Paging::default(&body.page)?;
    let list = db::msg::get_like_notices(&user, &client, &paging).await?;
    paging.finish(list)
}