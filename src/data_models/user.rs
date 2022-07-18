use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;
use tokio_postgres::Row;

#[derive(Debug, Deserialize, Serialize, PostgresMapper)]
#[pg_mapper(table = "users")]
pub struct User {
    pub id: i32,
    pub nick: String,
    pub pwd: Option<String>,
    pub _salt: String,
    pub avatar_url: Option<String>, // https://avatars.dicebear.com/api/pixel-art-neutral/123.svg
    pub create_time: NaiveDate,
}

impl From<&Row> for User {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get("id"),
            nick: row.get("nick"),
            pwd: row.get("pwd"),
            _salt: row.get("_salt"),
            avatar_url: row.get("avatar_url"),
            create_time: row.get("create_time"),
        }
    }
}

impl User {
    pub fn get_avatar_url(nick: &String) -> String {
        format!(
            "https://avatars.dicebear.com/api/{}/{}.svg",
            "pixel-art-neutral", nick
        )
    }
    pub fn to_user_data(&self) -> UserData {
        UserData::new(&self.id, &self.nick, self.avatar_url.clone())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserData {
    pub id: i32,
    pub nick: String,
    pub avatar_url: String,
}

impl UserData {
    pub fn new(id: &i32, nick: &String, avatar_url: Option<String>) -> Self {
        let avatar_url = avatar_url.unwrap_or(
            User::get_avatar_url(&nick)
        );
        Self {
            id: *id,
            nick: nick.clone(),
            avatar_url,
        }
    }
    pub fn optional(
        id: &Option<i32>,
        nick: &Option<String>,
        avatar_url: Option<String>,
    ) -> Option<Self> {
        if let Some(id) = id {
            if let Some(nick) = nick {
                return Some(Self::new(id, nick, avatar_url));
            }
        }
        None
    }
}

impl From<&Row> for UserData {
    fn from(row: &Row) -> Self {
        UserData::new(
            &row.get("id"),
            &row.get("nick"),
            row.try_get("avatar_url").ok(),
        )
    }
}