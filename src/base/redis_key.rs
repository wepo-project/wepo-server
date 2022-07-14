

/// REDIS 键
#[derive(Debug)]
pub struct RedisKey;


macro_rules! key_define {
    ($(
        $(#[$outer:meta])*
        ($i:ident, $($arg:ident$(,)?)+)
    ),* $(,)?) => {
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
        /// 点赞集合
        (post_likes, post_id),
        /// 点赞数量
        (post_like_count, post_id),
        /// 评论数组
        (post_comments, post_id),
        /// 评论数量
        (post_comments_count, post_id),
        /// 反感集合
        (post_hates, post_id),
        /// 反感集合
        (post_hate_count, post_id),
    }
}