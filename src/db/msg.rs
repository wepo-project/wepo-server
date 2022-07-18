use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

use crate::{
    base::{paging_data::Paging, user_info::UserInfo},
    data_models::notice::Notice,
    errors::MyError,
};

pub async fn get_notices<'a>(
    user: &UserInfo,
    client: &Client,
    paging: &Paging<'a>,
) -> Result<Vec<Notice>, MyError> {
    let _stmt = include_str!("../../sql/msg/get_notices.sql");
    let stmt = client.prepare(_stmt).await.map_err(MyError::PGError)?;

    Ok(client
        .query(&stmt, &[&user.id, paging.limit(), paging.offset()])
        .await?
        .iter()
        .map(|row| Notice::from_row_ref(row).unwrap())
        .collect::<Vec<Notice>>())
}
