use actix::Addr;
use actix_redis::RedisActor;
use async_trait::async_trait;

use crate::{base::user_info::UserInfo, errors::MyError};

#[async_trait]
pub trait SyncCache {
    async fn sync_cache_data(&mut self, user: Option<&UserInfo>, redis_addr: &Addr<RedisActor>) -> Result<(), MyError>;
}