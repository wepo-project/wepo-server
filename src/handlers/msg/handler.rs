use actix::Addr;
use actix_redis::RedisActor;
use actix_web::{web, HttpResponse};

use crate::{
    base::{
        paging_data::{GetPageDTO, Paging},
        pg_client::PGClient,
        user_info::UserInfo,
    },
    data_models::notice::NoticeType, utils::db_helper::RedisActorHelper,
};

use super::{storage, dto::UnreadMsg};

/// 获取评论通知
pub async fn get_comment_notices(
    user: UserInfo,
    body: web::Json<GetPageDTO>,
    client: PGClient,
    redis_addr: web::Data<Addr<RedisActor>>,
) -> Result<HttpResponse, actix_web::Error> {
    let paging = Paging::default(&body.page)?;
    let list = storage::get_comment_notices(&user, &paging, &client, &redis_addr).await?;
    paging.finish(list)
}

/// 获取评论通知
pub async fn get_like_notices(
    user: UserInfo,
    body: web::Json<GetPageDTO>,
    client: PGClient,
    redis_addr: web::Data<Addr<RedisActor>>,
) -> Result<HttpResponse, actix_web::Error> {
    let paging = Paging::default(&body.page)?;
    let list = storage::get_post_notices(&NoticeType::Like, &user, &paging, &client, &redis_addr).await?;
    paging.finish(list)
}

/// 获取评论通知
pub async fn get_hate_notices(
    user: UserInfo,
    body: web::Json<GetPageDTO>,
    client: PGClient,
    redis_addr: web::Data<Addr<RedisActor>>,
) -> Result<HttpResponse, actix_web::Error> {
    let paging = Paging::default(&body.page)?;
    let list = storage::get_post_notices(&NoticeType::Hate, &user, &paging, &client, &redis_addr).await?;
    paging.finish(list)
}

/// 获取好友添加通知
pub async fn get_add_friend_notices(
    user: UserInfo,
    body: web::Json<GetPageDTO>,
    client: PGClient,
    redis_addr: web::Data<Addr<RedisActor>>,
) -> Result<HttpResponse, actix_web::Error> {
    let paging = Paging::default(&body.page)?;
    let list = storage::get_friend_notices(&NoticeType::FriendAdd, &user, &paging, &client, &redis_addr).await?;
    paging.finish(list)
}

/// 获取好友移除通知
pub async fn get_remove_friend_notices(
    user: UserInfo,
    body: web::Json<GetPageDTO>,
    client: PGClient,
    redis_addr: web::Data<Addr<RedisActor>>,
) -> Result<HttpResponse, actix_web::Error> {
    let paging = Paging::default(&body.page)?;
    let list = storage::get_friend_notices(&NoticeType::FriendRemove, &user, &paging, &client, &redis_addr).await?;
    paging.finish(list)
}

/// 获取未读消息数量
pub async fn get_unread_msg(
    user: UserInfo,
    redis_addr: web::Data<Addr<RedisActor>>,
) -> Result<HttpResponse, actix_web::Error> {
    let list = redis_addr.exec_all(UnreadMsg::cmd_list(&user.id)).await?;
    let result = UnreadMsg::from_iter(list);
    Ok(HttpResponse::Ok().json(result))
}