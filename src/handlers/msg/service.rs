use crate::errors::MyError;

use super::data::NoticeType;

pub async fn send_notice_to_user(
    user_id: &i32,
    notice_type: NoticeType,
) -> Result<bool, MyError> {
    Ok(true)
}