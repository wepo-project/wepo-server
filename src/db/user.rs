use crate::{
    data_models::{User, UserData},
    errors::MyError,
    models::user::dto::{LoginUserDTO, RegisterUserDTO},
    utils,
};
use deadpool_postgres::Client;
use log::info;
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_postgres::error::SqlState;

/// 数据库添加用户
pub async fn add(client: &Client, mut user_info: RegisterUserDTO) -> Result<UserData, MyError> {
    let _stmt = include_str!("../../sql/user/add_user.sql");
    let stmt = client.prepare(&_stmt).await.map_err(MyError::PGError)?;

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
                    &SqlState::UNIQUE_VIOLATION => MyError::code(201),
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
        .ok_or(MyError::code(202))
}

/// 随机生成盐，长度为30
pub(crate) fn create_salt() -> String {
    utils::get_random_string(30)
}

/// 密码加密
/// sha256(sha256(pwd) + salt)
pub(crate) fn pwd_encrypt<S: Into<String>>(pwd: S, salt: &String) -> String {
    sha256::digest(format!("{}{}", sha256::digest(pwd), salt))
}

/// 验证用户
pub async fn validate_user(client: &Client, user_info: LoginUserDTO) -> Result<User, MyError> {
    let _stmt = include_str!("../../sql/user/get_user.sql");
    // let _stmt = _stmt.replace("$table_fields", &User::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.map_err(MyError::PGError)?;

    let user = client
        .query(&stmt, &[&user_info.nick])
        .await?
        .iter()
        .map(|row| User::from_row_ref(row).unwrap())
        .collect::<Vec<User>>()
        .pop()
        .ok_or(MyError::FailResultError)?; // 没有该用户

    if let Some(_pwd) = &user.pwd {
        if let Some(_raw_pwd) = &user_info.pwd {
            let _enc_pwd = pwd_encrypt(_raw_pwd, &user._salt);
            if _pwd.eq(&_enc_pwd) {
                // 密码正确
                Ok(user)
            } else {
                // 密码不相同202
                Err(MyError::code(202))
            }
        } else {
            // 没输入密码201
            Err(MyError::code(201))
        }
    } else {
        // 不用密码
        Ok(user)
    }
}

/// 修改昵称
pub async fn change_nick(client: &Client, id: &i32, nick: &String) -> Result<String, MyError> {
    let _stmt = include_str!("../../sql/user/change_nick.sql");
    let stmt = client.prepare(_stmt).await.map_err(MyError::PGError)?;
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
pub async fn search_user(client: &Client, nick: &String, page: &i64, limit: &i64) -> Result<Vec<UserData>, MyError> {
    let _stmt = include_str!("../../sql/user/search_user.sql");
    let stmt = client.prepare(_stmt).await.map_err(MyError::PGError)?;
    let _offset: i64 = limit * (page - 1);
    Ok(client
        .query(&stmt, &[&format!("%{}%", nick), limit, &_offset])
        .await?
        .iter()
        .map(|row| UserData::from(row))
        .collect::<Vec<UserData>>())
}

#[cfg(test)]
mod test {
    use crate::db::user::create_salt;

    #[test]
    fn salt() {
        println!("{}", create_salt());
    }
}
