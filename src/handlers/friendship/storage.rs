
use tokio_postgres::error::SqlState;

use crate::{
    base::{pg_client::PGClient, user_info::UserInfo},
    errors::MyError,
};

/// 201 已经添加
pub async fn add_friend(
    user: &UserInfo,
    addressee_id: &i32,
    client: &PGClient,
) -> Result<(), MyError> {
    let _stmt = include_str!("../../../sql/friendship/add_friendship.sql");
    let stmt = client.prepare(_stmt).await?;

    let _v = client
        .query(&stmt, &[&user.id, addressee_id])
        .await
        .map_err(|e| {
            if let Some(state) = e.code() {
                match state {
                    &SqlState::UNIQUE_VIOLATION => MyError::err_code(201),
                    _ => MyError::PGError(e),
                }
            } else {
                MyError::PGError(e)
            }
        })?;
    Ok(())
}

pub async fn remove_friend(
    user: &UserInfo,
    addressee_id: &i32,
    client: &PGClient,
) -> Result<(), MyError> {
    let _stmt = include_str!("../../../sql/friendship/remove_friendship.sql");
    let stmt = client.prepare(_stmt).await?;
    let _row = client.query(&stmt, &[&user.id, addressee_id])
        .await
        .map_err(MyError::PGError)?;
    Ok(())
}
