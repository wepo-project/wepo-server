use uuid::Uuid;

/// REDIS 键
#[derive(Debug)]
pub struct PostRedisKey {
    pub likes: String,
    pub likes_count: String,
}

impl PostRedisKey {
    pub fn new(post_id: &Uuid) -> Self {
        Self {
            likes: format!("post_like:{}", post_id),
            likes_count: format!("post_like_count:{}", post_id),
        }
    }
}