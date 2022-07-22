use std::str::FromStr;

use actix::Addr;
use actix_redis::{resp_array, Command, RedisActor, RespValue};
use async_trait::async_trait;
use futures::future::try_join_all;
use redis_async::resp::FromResp;

use crate::errors::MyError;

#[async_trait]
pub trait RedisActorHelper {
    async fn exec(&self, command: Command) -> Result<RespValue, MyError>;
    async fn exec_all(&self, commands: Vec<Command>) -> Result<Vec<RespValue>, MyError>;
    async fn get_i64(&self, key: &String) -> Result<i64, MyError>;
    fn do_send_all(&self, commands: Vec<Command>) -> ();
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
    fn do_send_all(&self, commands: Vec<Command>) -> () {
        for cmd in commands {
            self.do_send(cmd);
        }
    }
    async fn get_i64(&self, key: &String) -> Result<i64, MyError> {
        let result = self.exec(RedisCmd::get(key)).await?;
        let int = String::from_resp(result)
            .map_err(|_e| MyError::ParseError)?
            .parse::<i64>()
            .map_err(|_e| MyError::ParseError)?;
        Ok(int)
    }
    fn del(&self, key: &String) -> () {
        self.do_send(RedisCmd::del(key));
    }
}


macro_rules! cmd_define {
    ($(
        $(#[$outer:meta])*
        ($i:ident, $($arg:ident$(,)?)+)
    ),* $(,)?) => {
        $(
            $(#[$outer])*
            #[allow(dead_code)]
            pub fn $i($($arg: impl Into<RespValue>,)*) -> Command {
                Command(resp_array![stringify!($i), $($arg,)*])
            }
        )*
    };
}

pub struct RedisCmd;

impl RedisCmd {
    cmd_define! {
        /// 获取
        (get, key),
        /// 设置
        (set, key, value),
        /// 删除
        (del, key),
        /// 自增
        (incr, key),
        /// 自减
        (decr, key),
        /// 集合增加
        (sadd, key, member),
        /// 是否在集合中
        (sismember, key, member),
        /// 集合移除
        (srem, key, member),
        /// 数组push
        (lpush, key, value),
        /// 设置过期
        (expire, key, seconds),
    }
}

pub trait RespValueRedisHelper {
    fn integer_to_bool(&self) -> bool;
    fn integer_to_i64(&self) -> i64;
    fn bulk_to_num<N: FromStr>(&self) -> Option<N>;
}

impl RespValueRedisHelper for RespValue {
    fn integer_to_bool(&self) -> bool {
        match self {
            RespValue::Integer(num) => num == &1,
            _ => false,
        }
    }
    fn integer_to_i64(&self) -> i64 {
        match self {
            RespValue::Integer(num) => *num,
            _ => 0,
        }
    }
    fn bulk_to_num<N: FromStr>(&self) -> Option<N> {
        if let RespValue::BulkString(bytes) = self {
            if let Ok(num) = String::from_utf8_lossy(bytes).into_owned().parse::<N>() {
                return Some(num);
            }
        }
        None
    }
}
