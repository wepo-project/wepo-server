
use log::info;

use crate::{
    base::{paging_data::Paging, user_info::UserInfo, pg_client::PGClient},
    data_models::notice::{NoticeType, NoticeComment, NoticePost, NoticeFriend},
    errors::MyError,
};

/// 发送通知
pub async fn send_notice(
    sender: &i32, 
    notice_type: &NoticeType, 
    sender_object: &String, 
    addressee_id: &i32,
    client: &PGClient,
) -> Result<(), MyError> {
    let _stmt = include_str!("../../../sql/msg/insert_notices.sql");
    let stmt = client.prepare(_stmt).await?;

    let result = client
        .query(&stmt, &[sender, notice_type.to_i16(), sender_object, addressee_id])
        .await?
        .iter()
        .map(|_| ())
        .collect::<Vec<()>>()
        .pop()
        .ok_or(MyError::InternalServerError);

    if let Err(ref e) = result {
        info!("send notice error: {}", e);
    }
    result
}

/// 获取评论通知
pub async fn get_comment_notices<'a>(
    user: &UserInfo,
    client: &PGClient,
    paging: &Paging<'a>,
) -> Result<Vec<NoticeComment>, MyError> {
    let _stmt = include_str!("../../../sql/msg/get_comment_notices.sql");
    let stmt = client.prepare(_stmt).await?;
    // 未读消息，需要设置 read: true
    let mut unread_vec = vec![];

    let vec = client
    .query(&stmt, &[
        NoticeType::Comment.to_i16(),
        &user.id, 
        paging.limit(), 
        paging.offset()
    ])
    .await?
    .iter()
    .map(|row| {
        let notice = NoticeComment::from(row);
        if !notice.read {
            unread_vec.push(notice.id);
        }
        notice
    })
    .collect::<Vec<NoticeComment>>();

    Ok(vec)
}


/// 获取点赞通知
pub async fn get_post_notices<'a>(
    notice_type: &NoticeType,
    user: &UserInfo,
    client: &PGClient,
    paging: &Paging<'a>,
) -> Result<Vec<NoticePost>, MyError> {
    let _stmt = include_str!("../../../sql/msg/get_post_notices.sql");
    let stmt = client.prepare(_stmt).await?;
    // 未读消息，需要设置 read: true
    let mut unread_vec = vec![];

    let vec = client
    .query(&stmt, &[
        notice_type.to_i16(),
        &user.id, 
        paging.limit(), 
        paging.offset()
    ])
    .await?
    .iter()
    .map(|row| {
        let notice = NoticePost::from(row);
        if !notice.read {
            unread_vec.push(notice.id);
        }
        notice
    })
    .collect::<Vec<NoticePost>>();

    Ok(vec)
}


/// 获取好友通知
pub async fn get_friend_notices<'a>(
    notice_type: &NoticeType,
    user: &UserInfo,
    client: &PGClient,
    paging: &Paging<'a>,
) -> Result<Vec<NoticeFriend>, MyError> {
    let _stmt = include_str!("../../../sql/msg/get_friend_notices.sql");
    let stmt = client.prepare(_stmt).await?;
    // 未读消息，需要设置 read: true
    let mut unread_vec = vec![];

    let vec = client
    .query(&stmt, &[
        notice_type.to_i16(),
        &user.id, 
        paging.limit(), 
        paging.offset()
    ])
    .await?
    .iter()
    .map(|row| {
        let notice = NoticeFriend::from(row);
        if !notice.read {
            unread_vec.push(notice.id);
        }
        notice
    })
    .collect::<Vec<NoticeFriend>>();

    Ok(vec)
}