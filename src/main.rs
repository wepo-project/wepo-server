mod base;
mod config;
mod data_models;
mod db;
mod errors;
mod handlers;
mod utils;
mod traits;

use crate::config::WepoConfig;
use crate::{
    handlers::UserHandler,
    handlers::PostHandler,
    handlers::MsgHandler,
};
use ::config::Config;
use actix_cors::Cors;
use actix_redis::RedisActor;
use actix_web::{http, web::{self, get, post, delete}, App, HttpServer};
use dotenv::dotenv;
use log::info;
use tokio_postgres::NoTls;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_logger();

    dotenv().ok();

    let config_ = Config::builder()
        .add_source(::config::Environment::default())
        .build()
        .unwrap();

    let config: WepoConfig = config_.try_deserialize().unwrap();
    let redis_addr = RedisActor::start(config.redis_addr.clone());
    let pool = config.pg.create_pool(None, NoTls).unwrap();

    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "DELETE", "PUT"])
            .allowed_headers(vec![
                http::header::AUTHORIZATION,
                http::header::ACCEPT,
                http::header::CONTENT_TYPE,
            ])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(redis_addr.clone()))
            .service(
                web::scope("/v1")
                    .service(
                        web::scope("/user")
                            .route("/register", post().to(UserHandler::register)) // 注册
                            .route("/login", post().to(UserHandler::login)) // 账号密码登录
                            .route("/token_login", get().to(UserHandler::login_with_token)) // token 登录
                            .route("/change_nick", post().to(UserHandler::change_nick)) // 修改昵称
                            .route("/search_user", post().to(UserHandler::search_user)) // 查找用户
                    )
                    .service(
                        web::scope("/post")
                            .route("/send", post().to(PostHandler::add)) // 发送
                            .route("/delete", delete().to(PostHandler::delete)) // 删除
                            .route("/like", get().to(PostHandler::like)) // 喜欢
                            .route("/cancel_like", get().to(PostHandler::cancel_like)) // 取消喜欢
                            .route("/hate", get().to(PostHandler::hate)) // 反感
                            .route("/cancel_hate", get().to(PostHandler::cancel_hate)) // 取消反感
                            .route("/get_post", get().to(PostHandler::get_one)) // 获取某个
                            .route("/my_post", post().to(PostHandler::mine)) // 获取我的（翻页）
                            .route("/comment", post().to(PostHandler::comment)) // 评论
                            .route("/browse", get().to(PostHandler::browse)) // 浏览所有（翻页）
                    )
                    .service(
                        web::scope("/msg")
                        .route("/notices", post().to(MsgHandler::get_notices)), // 获取通知
                    )
            )
    })
    .bind(config.server_addr.clone())?
    .run();

    info!("Server running at http://{}/", config.server_addr);

    server.await
}

/// 初始化日志
pub fn init_logger() {
    use chrono::Local;
    use std::io::Write;

    let env = env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info");
    // 设置打印日志的格式
    env_logger::Builder::from_env(env)
        .format(|buf, record| {
            writeln!(
                buf,
                "{} {} [{}] {}",
                Local::now().format("%Y-%m-%d %H:%m:%S"),
                record.level(),
                record.module_path().unwrap_or("<unnamed>"),
                &record.args()
            )
        })
        .init();
    info!("env_logger initialized.");
}
