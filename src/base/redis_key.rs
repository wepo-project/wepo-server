

/// REDIS 键
#[derive(Debug)]
pub struct PostRedisKey;

impl PostRedisKey {
    // 点赞集合
    pub fn likes(post_id: &i64) -> String {
        format!("post_like:{}", post_id)
    }
    /// 点赞数量
    pub fn likes_count(post_id: &i64) -> String {
        format!("post_like_count:{}", post_id)
    }
    /// 评论数组
    pub fn comments(post_id: &i64) -> String {
        format!("comments:{}", post_id)
    }
    /// 评论数量
    pub fn comments_count(post_id: &i64) -> String {
        format!("comment_count:{}", post_id)
    }
}