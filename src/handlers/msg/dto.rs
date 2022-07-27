
use actix_redis::{Command, RespValue};
use serde::{Deserialize, Serialize};

use crate::{
    base::redis_key::RedisKey,
    utils::db_helper::{RedisCmd, RespValueRedisHelper},
};

/// 未读消息
#[derive(Serialize, Deserialize)]
pub struct UnreadMsg {
    comments: i32,
    likes: i32,
    hates: i32,
    friend_add: i32,
    friend_remove: i32,
}

impl UnreadMsg {
    pub fn cmd_list(user_id: &i32) -> Vec<Command> {
        vec![
            RedisCmd::get(RedisKey::unread_comments(user_id)),
            RedisCmd::get(RedisKey::unread_likes(user_id)),
            RedisCmd::get(RedisKey::unread_hates(user_id)),
            RedisCmd::get(RedisKey::unread_friend_add(user_id)),
            RedisCmd::get(RedisKey::unread_friend_remove(user_id)),
        ]
    }
}

impl FromIterator<RespValue> for UnreadMsg {
    fn from_iter<T: IntoIterator<Item = RespValue>>(iter: T) -> Self {
        let iter = iter.into_iter();
        let mut msg = Self::default();
        let ptr_list = [
            &mut msg.comments,
            &mut msg.likes,
            &mut msg.hates,
            &mut msg.friend_add,
            &mut msg.friend_remove,
        ];
        iter.enumerate().for_each(|(idx, val)| {
            if let Some(num) = val.bulk_to_num::<i32>() {
                *ptr_list[idx] = num;
            }
        });
        msg
    }
}

impl Default for UnreadMsg {
    fn default() -> Self {
        Self {
            comments: Default::default(),
            likes: Default::default(),
            hates: Default::default(),
            friend_add: Default::default(),
            friend_remove: Default::default(),
        }
    }
}
