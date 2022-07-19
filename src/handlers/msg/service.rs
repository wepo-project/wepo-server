use crate::{
    base::{big_int::BigInt, pg_client::PGClient},
    data_models::notice::NoticeType,
    db,
    errors::MyError,
};

/// 发送评论通知
pub async fn send_comment_notice(
    sender_id: &i32,
    receiver_id: &i32,
    post_id: &BigInt,
    client: &PGClient,
) -> Result<(), MyError> {
    db::msg::send_notice(
        sender_id,
        &NoticeType::Comment,
        &post_id.to_string(),
        receiver_id,
        &client,
    )
    .await?;
    Ok(())
}
