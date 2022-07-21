use actix_web::{web, HttpResponse};

use crate::{
    base::{
        paging_data::{GetPageDTO, Paging},
        pg_client::PGClient,
        user_info::UserInfo,
    },
    data_models::notice::NoticeType,
    db,
};

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
    let list = db::msg::get_post_notices(&NoticeType::Like, &user, &client, &paging).await?;
    paging.finish(list)
}

/// 获取评论通知
pub async fn get_hate_notices(
    user: UserInfo,
    body: web::Json<GetPageDTO>,
    client: PGClient,
) -> Result<HttpResponse, actix_web::Error> {
    let paging = Paging::default(&body.page)?;
    let list = db::msg::get_post_notices(&NoticeType::Hate, &user, &client, &paging).await?;
    paging.finish(list)
}
