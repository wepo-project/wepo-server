use deadpool_postgres::Client;
use tokio_postgres::Row;

use crate::{base::user_info::UserInfo, models::post::dto::*, data_models::Post, errors::MyError};

use tokio_pg_mapper::FromTokioPostgresRow;


pub(crate) async fn add_post(
  user: &UserInfo, 
  post_data: &AddPostDTO,
  client: &Client,
) -> Result<Post, MyError> {
  let _stmt = include_str!("../../sql/add_post.sql");
  let _stmt = _stmt.replace("$table_fields", &Post::sql_table_fields());
  let stmt = client.prepare(&_stmt).await.unwrap();

  client.query(&stmt, &[&user.id, &post_data.content])
    .await?
    .iter()
    .map(|row| Post::from_row_ref(row).unwrap())
    .collect::<Vec<Post>>()
    .pop()
    .ok_or(MyError::NotFound)
}

pub(crate) async fn del_post(
  user: &UserInfo, 
  del_data: &DelPostDTO,
  client: &Client,
) -> Result<(), MyError> {
  let _stmt = include_str!("../../sql/delete_post.sql");
  let stmt = client.prepare(_stmt).await.unwrap();

  client
    .query(&stmt, &[&del_data.id, &user.id])
    .await
    .map(|_| ())
    .map_err(MyError::PGError)
}