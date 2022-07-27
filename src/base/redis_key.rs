

/// REDIS 键
#[derive(Debug)]
pub struct RedisKey;


macro_rules! key_define {
    ($(
        $(#[$outer:meta])*
        $i:ident => $($arg:ident$(,)?)+
    )*$(,)?) => {
        $(
            $(#[$outer])*
            pub fn $i<S: ToString>($($arg: S,)*) -> String {
                format!("{}:{}", stringify!($i), $($arg.to_string(),)*)
            }
        )*
    };
}

impl RedisKey {
    key_define! {
        // ============【 Post 】============
        /// 点赞集合 SET
        post_likes => post_id,
        /// 点赞数量 NUMBER
        post_like_count => post_id,
        /// 反感集合 SET
        post_hates => post_id,
        /// 反感数量 NUMBER
        post_hate_count => post_id,
        /// 获取post的发送者 STRING
        post_sender => post_id,

        // ============【 未读 】============
        // /// 所有未读总数
        // unread_total => user_id,
        /// 未读评论 NUMBER
        unread_comments => user_id,
        /// 未读点赞 NUMBER
        unread_likes => user_id,
        /// 未读反感 NUMBER
        unread_hates => user_id,
        /// 未读好友添加 NUMBER
        unread_friend_add => user_id,
        /// 未读好友移除 NUMBER
        unread_friend_remove => user_id
    }
}