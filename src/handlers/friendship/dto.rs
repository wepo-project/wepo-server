use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct SendFriendRequestDTO {
    pub user_id: i32,
    pub msg: String,
}