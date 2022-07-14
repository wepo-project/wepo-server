use actix::Addr;
use actix_redis::{resp_array, Command, RedisActor, RespValue};
use async_trait::async_trait;
use futures::future::try_join_all;
use redis_async::resp::FromResp;

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
        let result = self.exec(RedisCmd::get(key)).await?;
        let int = String::from_resp(result)
            .map_err(|_e| MyError::ParseError)?
            .parse::<i64>()
            .map_err(|_e| MyError::ParseError)?;
        Ok(int)
    }
    fn del(&self, key: &String) -> () {
        self.do_send(cmd!["DEL", key]);
    }
}


macro_rules! cmd_define {
    ($(
        $(#[$outer:meta])*
        ($i:ident, $($arg:ident$(,)?)+)
    ),* $(,)?) => {
        $(
            $(#[$outer])*
            pub fn $i($($arg: &String,)*) -> Command {
                cmd![stringify!($i), $($arg,)*]
            }
        )*
    };
}

pub struct RedisCmd;

impl RedisCmd {
    cmd_define! {
        /// 获取
        (get, key),
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
    }
}

pub trait RespValueRedisHelper {
    fn integer_to_bool(&self) -> bool;
    fn integer_to_i64(&self) -> i64;
    fn bulk_to_i64(&self) -> Option<i64>;
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
    fn bulk_to_i64(&self) -> Option<i64> {
        if let RespValue::BulkString(bytes) = self {
            if let Ok(num) = String::from_utf8_lossy(bytes).into_owned().parse::<i64>() {
                return Some(num);
            }
        }
        None
    }
}
