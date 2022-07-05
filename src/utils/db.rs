use actix::Addr;
use actix_redis::{resp_array, Command, RedisActor, RespValue};
use async_trait::async_trait;

use crate::errors::MyError;

#[async_trait]
pub trait RedisActorHelper {
    async fn exec(&self, cmd: Command) -> Result<RespValue, MyError>;
    async fn sismember(&self, key: &String, member: &String) -> Result<bool, MyError>;
}

#[async_trait]
impl RedisActorHelper for Addr<RedisActor> {
    async fn exec(&self, cmd: Command) -> Result<RespValue, MyError> {
        self.send(cmd)
            .await
            .map_err(MyError::MailboxError)?
            .map_err(MyError::RedisError)
    }
    async fn sismember(&self, key: &String, member: &String) -> Result<bool, MyError> {
        self.exec(Command(resp_array!["SISMEMBER", key, member]))
            .await
            .map(|resp| {
                if let RespValue::Integer(ret) = resp {
                    ret == 1
                } else {
                    false
                }
            })
    }
}
