use actix::Addr;
use actix_redis::{resp_array, Command, RedisActor, RespValue};
use async_trait::async_trait;
use futures::future::try_join_all;

use crate::errors::MyError;

macro_rules! cmd {
    ($($e:expr),* $(,)?) => {
        Command(resp_array![$($e),*])
    };
}

#[async_trait]
pub trait RedisActorHelper {
    async fn exec(&self, command: Command) -> Result<RespValue, MyError>;
    async fn exec_all(&self, commands: Vec<Command>) -> Result<Vec<RespValue>, MyError>;
    async fn get_i64(&self, key: &String) -> Result<i64, MyError>;
    fn del(&self, key: &String) -> ();
}

#[async_trait]
impl RedisActorHelper for Addr<RedisActor> {
    async fn exec(&self, command: Command) -> Result<RespValue, MyError> {
        self.send(command)
            .await
            .map_err(MyError::MailboxError)?
            .map_err(MyError::RedisError)
    }
    async fn exec_all(&self, commands: Vec<Command>) -> Result<Vec<RespValue>, MyError> {
        try_join_all(commands.into_iter().map(|command| self.exec(command))).await
    }
    async fn get_i64(&self, key: &String) -> Result<i64, MyError> {
        let result = self.exec(cmd!["GET", key]).await?;
        if let RespValue::BulkString(utf8_arr) = result {
            // 成功获取到需要转换成字符串，然后转换成i64
            let str = String::from_utf8(utf8_arr);
            if let Ok(str) = str {
                let likes = str.parse::<i64>();
                if let Ok(likes) = likes {
                    return Ok(likes);
                }
            }
        }
        Err(MyError::NotFound)
    }
    fn del(&self, key: &String) -> () {
        self.do_send(cmd!["DEL", key]);
    }
}

pub struct RedisCmd;

impl RedisCmd {
    // 增加
    pub fn incr(key: &String) -> Command {
        cmd!["INCR", key]
    }
    // 减少
    pub fn decr(key: &String) -> Command {
        cmd!["DECR", key]
    }
    // 集合增加
    pub fn sadd(key: &String, member: &String) -> Command {
        cmd!["SADD", key, member]
    }
    // 是否在集合中
    pub fn sismember(key: &String, member: &String) -> Command {
        cmd!["SISMEMBER", key, member]
    }
    // 集合移除
    pub fn srem(key: &String, member: &String) -> Command {
        cmd!["SREM", key, member]
    }
}


pub trait RespValueRedisHelper {
    fn int_to_bool(&self) -> bool;
    fn to_i64(&self) -> i64;
}

impl RespValueRedisHelper for RespValue {
    fn int_to_bool(&self) -> bool {
        match self {
            RespValue::Integer(num) => num == &1,
            _ => false,
        }
    }
    fn to_i64(&self) -> i64 {
        match self {
            RespValue::Integer(num) => *num,
            _ => 0,
        }
    }
}
