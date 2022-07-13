use actix::Addr;
use actix_redis::RedisActor;
use async_trait::async_trait;

use crate::base::user_info::UserInfo;

#[async_trait]
pub trait SyncCache {
    async fn sync_cache_data(&mut self, user: &UserInfo, redis_addr: &Addr<RedisActor>) -> ();
}