use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct RegisterUserDTO {
    pub nick: String,
    /// 没有密码的话，谁都能登录
    pub pwd: Option<String>,
}