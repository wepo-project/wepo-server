use crate::{dto::user::RegisterUserDTO, errors::MyError, models::User};
use deadpool_postgres::Client;
use log::info;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use tokio_pg_mapper::FromTokioPostgresRow;

pub async fn add_user(client: &Client, mut user_info: RegisterUserDTO) -> Result<User, MyError> {
    let _stmt = include_str!("../../sql/add_user.sql");
    let _stmt = _stmt.replace("$table_fields", &User::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();

    if let Some(_pwd) = &user_info.pwd {
        user_info.pwd = Some(sha256::digest(_pwd));
    }

    let _salt = create_salt().unwrap();

    info!("salt:{}", _salt);

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

#[cfg(test)]
mod test {
    use crate::db::user::create_salt;

    #[test]
    fn salt() {
        println!("{}", create_salt().unwrap());
    }
}
