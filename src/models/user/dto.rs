use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct RegisterUserDTO {
    pub nick: String,
    /// 没有密码的话，谁都能登录
    pub pwd: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct RegisterResultDTO {
    pub id: i32,
    pub nick: String,
}


#[derive(Deserialize, Serialize)]
pub struct LoginUserDTO {
    pub nick: String,
    pub pwd: Option<String>,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct LoginResultDTO {
    pub id: i32,
    pub nick: String,
    pub token: String,
}

#[derive(Deserialize, Serialize)]
pub struct ChangeNickDTO {
    pub nick: String,
}
