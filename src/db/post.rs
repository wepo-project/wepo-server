use actix::Addr;
use actix_redis::{resp_array, Command, RedisActor, RespValue};
use deadpool_postgres::Client;
use futures::future::try_join;
use log::info;
use uuid::Uuid;

use crate::{
    base::user_info::UserInfo,
    data_models::Post,
    errors::MyError,
    models::post::dto::*,
};

use tokio_pg_mapper::FromTokioPostgresRow;

/// 添加
pub async fn add_post(
    user: &UserInfo,
    post_data: &AddPostDTO,
    client: &Client,
) -> Result<Post, MyError> {
    let _stmt = include_str!("../../sql/post/add_post.sql");
    let _stmt = _stmt.replace("$table_fields", &Post::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();

    client
        .query(&stmt, &[&user.id, &post_data.content])
        .await?
        .iter()
        .map(|row| Post::from_row_ref(row).unwrap())
        .collect::<Vec<Post>>()
        .pop()
        .ok_or(MyError::NotFound)
}

/// 删除推文
pub async fn del_post(
    user: &UserInfo,
    del_data: &DelPostDTO,
    client: &Client,
    redis_addr: &Addr<RedisActor>,
) -> Result<(), MyError> {
    let _stmt = include_str!("../../sql/post/delete_post.sql");
    let stmt = client.prepare(_stmt).await.unwrap();

    client
        .query(&stmt, &[&del_data.id, &user.id])
        .await
        .map(|_| {
            // 删除post的redis缓存数据
            let key = PostRedisKey::new(&del_data.id);
            redis_addr.do_send(Command(resp_array!["DEL", key.likes]));
            redis_addr.do_send(Command(resp_array!["DEL", key.likes_count]));
        })
        .map_err(MyError::PGError)
}

pub async fn get_post(
    post_id: &Uuid,
    client: &Client,
    redis_addr: &Addr<RedisActor>,
) -> Result<Post, MyError> {
    let _stmt = include_str!("../../sql/post/get_post.sql");
    let stmt = client.prepare(_stmt).await.unwrap();
    let mut post = client
        .query(&stmt, &[&post_id.hyphenated().to_string()])
        .await?
        .iter()
        .map(|row| Post::from_row_ref(row).unwrap())
        .collect::<Vec<Post>>()
        .pop()
        .ok_or(MyError::NotFound)?;

    info!("{:?}", post);

    let key = PostRedisKey::new(post_id);
    let val = redis_addr
        .send(Command(resp_array!["GET", &key.likes_count]))
        .await
        .map_err(MyError::MailboxError)?
        .map_err(MyError::RedisError)?;
    info!("{:?}", val);

    if let RespValue::Integer(num) = val {
      post.likes = num as i32;
    }

    Ok(post)
}

/// 点赞
/// 用户点赞 数据结构
/// key: post_like:<post_id>
/// value: {user_id}
/// 点赞数
/// key: post_like_count:<post_id>
/// value: likes count
pub async fn like_post(
    post_id: &Uuid,
    user_id: &i32,
    redis_addr: &Addr<RedisActor>,
) -> Result<(), MyError> {
    let key = PostRedisKey::new(post_id);
    let ret = try_join(
        redis_addr.send(Command(resp_array![
            "SADD",
            key.likes,
            &user_id.to_string()
        ])),
        redis_addr.send(Command(resp_array!["INCR", key.likes_count])),
    )
    .await
    .map_err(MyError::MailboxError)?;

    if let Err(e) = ret.0 {
        Err(MyError::RedisError(e))
    } else if let Err(e) = ret.1 {
        Err(MyError::RedisError(e))
    } else {
        Ok(())
    }
}

/// 取消点赞
pub async fn unlike_post(
    post_id: &Uuid,
    user_id: &i32,
    redis_addr: &Addr<RedisActor>,
) -> Result<(), MyError> {
    let key = PostRedisKey::new(post_id);
    let cmd1 = redis_addr.send(Command(resp_array![
        "SREM",
        key.likes,
        &user_id.to_string()
    ]));
    let cmd2 = redis_addr.send(Command(resp_array!["DECR", key.likes_count]));

    let ret = try_join(cmd1, cmd2).await.map_err(MyError::MailboxError)?;

    if let Err(e) = ret.0 {
        Err(MyError::RedisError(e))
    } else if let Err(e) = ret.1 {
        Err(MyError::RedisError(e))
    } else {
        Ok(())
    }
}

/// REDIS 键
struct PostRedisKey {
    pub likes: String,
    pub likes_count: String,
}

impl PostRedisKey {
    fn new(post_id: &Uuid) -> Self {
        Self {
            likes: format!("post_like:{}", post_id),
            likes_count: format!("post_like_count:{}", post_id),
        }
    }
}
