use crate::dto::user::LoginUserDTO;
use crate::{dto::user::RegisterUserDTO, errors::MyError, models::User};
use deadpool_postgres::Client;
use log::info;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use tokio_pg_mapper::FromTokioPostgresRow;

/// 数据库添加用户
pub async fn add_user(client: &Client, mut user_info: RegisterUserDTO) -> Result<User, MyError> {
    let _stmt = include_str!("../../sql/add_user.sql");
    let _stmt = _stmt.replace("$table_fields", &User::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();

    let _salt = create_salt().unwrap();

    if let Some(_pwd) = &user_info.pwd {
        user_info.pwd = Some(pwd_encrypt(_pwd, &_salt));
    }

    info!("creating a new user:{}", user_info.nick);

    client
        .query(&stmt, &[&user_info.nick, &user_info.pwd, &_salt])
        .await?
        .iter()
        .map(|row| User::from_row_ref(row).unwrap())
        .collect::<Vec<User>>()
        .pop()
        .ok_or(MyError::NotFound) // more applicable for SELECTs
}

/// 随机生成盐，长度为30
pub(crate) fn create_salt() -> Result<String, std::string::FromUtf8Error> {
    String::from_utf8(thread_rng().sample_iter(&Alphanumeric).take(30).collect())
}

/// 密码加密
/// sha256(sha256(pwd) + salt)
pub(crate) fn pwd_encrypt<S: Into<String>>(pwd: S, salt: &String) -> String {
    sha256::digest(format!("{}{}", sha256::digest(pwd), salt))
}

/// 验证用户
pub async fn validate_user(client: &Client, user_info: LoginUserDTO) -> Result<User, MyError> {
    let _stmt = include_str!("../../sql/get_user.sql");
    // let _stmt = _stmt.replace("$table_fields", &User::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();


    let user = client
        .query(&stmt, &[&user_info.nick])
        .await?
        .iter()
        .map(|row| User::from_row_ref(row).unwrap())
        .collect::<Vec<User>>()
        .pop()
        .ok_or(MyError::NotFound)?;

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

#[cfg(test)]
mod test {
    use crate::db::user::create_salt;

    #[test]
    fn salt() {
        println!("{}", create_salt().unwrap());
    }
}
