use actix::Addr;
use actix_redis::RedisActor;
use async_trait::async_trait;

#[async_trait]
pub trait SyncCache {
    async fn sync_cache_data(&mut self, redis_addr: &Addr<RedisActor>) -> ();
}