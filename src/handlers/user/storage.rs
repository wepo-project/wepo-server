use crate::{
    data_models::user::{User, UserData},
    errors::MyError,
    handlers::user::dto::{LoginUserDTO, RegisterUserDTO},
    utils, base::{paging_data::Paging, pg_client::PGClient},
};
use log::info;
use tokio_postgres::error::SqlState;

/// 数据库添加用户
pub async fn add(client: &PGClient, mut user_info: RegisterUserDTO) -> Result<UserData, MyError> {
    let _stmt = include_str!("../../../sql/user/add_user.sql");
    let stmt = client.prepare(&_stmt).await?;

    let _salt = create_salt();
    if let Some(_pwd) = &user_info.pwd {
        user_info.pwd = Some(pwd_encrypt(_pwd, &_salt));
    }

    client
        .query(&stmt, &[&user_info.nick, &user_info.pwd, &_salt])
        .await
        .map_err(|e| {
            info!("{}", e);
            if let Some(state) = e.code() {
                match state {
                    // 名字重复
                    &SqlState::UNIQUE_VIOLATION => MyError::err_code(201),
                    _ => MyError::PGError(e),
                }
            } else {
                MyError::PGError(e)
            }
        })? // 注册失败
        .iter()
        .map(|row| UserData::new(&row.get("id"), &row.get("nick"), None))
        .collect::<Vec<UserData>>()
        .pop()
        .ok_or(MyError::err_code(202))
}

/// 随机生成盐，长度为30
pub fn create_salt() -> String {
    utils::get_random_string(30)
}

/// 密码加密
/// sha256(sha256(pwd) + salt)
pub fn pwd_encrypt<S: Into<String>>(pwd: S, salt: &String) -> String {
    sha256::digest(format!("{}{}", sha256::digest(pwd), salt))
}

/// 验证用户
/// 203 没有该用户
/// 202 密码错误
/// 201 没输入密码
pub async fn validate_user(client: &PGClient, user_info: LoginUserDTO, from_token: bool) -> Result<UserData, MyError> {
    let _stmt = include_str!("../../../sql/user/get_user_from_nick.sql");
    let stmt = client.prepare(&_stmt).await?;

    let user = client
        .query(&stmt, &[&user_info.nick])
        .await?
        .iter()
        .map(|row| User::from(row))
        .collect::<Vec<User>>()
        .pop()
        .ok_or(MyError::err_code(203))?; // 没有该用户

    if from_token {
        return Ok(user.to_user_data());
    }

    if let Some(_pwd) = &user.pwd {
        if let Some(_raw_pwd) = &user_info.pwd {
            let _enc_pwd = pwd_encrypt(_raw_pwd, &user._salt);
            if _pwd.eq(&_enc_pwd) {
                // 密码正确
                Ok(user.to_user_data())
            } else {
                // 密码不相同202
                Err(MyError::err_code(202))
            }
        } else {
            // 没输入密码201
            Err(MyError::err_code(201))
        }
    } else {
        // 不用密码
        Ok(user.to_user_data())
    }
}

/// 修改昵称
pub async fn change_nick(client: &PGClient, id: &i32, nick: &String) -> Result<String, MyError> {
    let _stmt = include_str!("../../../sql/user/change_nick.sql");
    let stmt = client.prepare(_stmt).await?;
    client
        .query(&stmt, &[id, nick])
        .await?
        .iter()
        .map(|r| r.get("nick"))
        .collect::<Vec<String>>()
        .pop()
        .ok_or(MyError::FailResultError)
}

/// 搜索用户
pub async fn search_user<'a>(client: &PGClient, nick: &String, paging: &Paging<'a>) -> Result<Vec<UserData>, MyError> {
    let _stmt = include_str!("../../../sql/user/search_user.sql");
    let stmt = client.prepare(_stmt).await?;
    Ok(client
        .query(&stmt, &[&format!("%{}%", nick), paging.limit(), paging.offset()])
        .await?
        .iter()
        .map(|row| UserData::from(row))
        .collect::<Vec<UserData>>())
}