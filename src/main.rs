mod base;
mod config;
mod data_models;
mod db;
mod errors;
mod models;
mod utils;
mod traits;

use crate::config::WepoConfig;
use crate::{
    models::user::handler as UserHandler,
    models::post::handler as PostHandler,
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
                        web::scope("/token")
                            .route("/login", get().to(UserHandler::login_with_token)),
                    )
                    .service(
                        web::scope("/user")
                            .route("/register", post().to(UserHandler::register))
                            .route("/login", post().to(UserHandler::login))
                            .route("/change_nick", post().to(UserHandler::change_nick))
                            .route("/search_user", post().to(UserHandler::search_user)),
                    )
                    .service(
                        web::scope("/post")
                            .route("/send", post().to(PostHandler::add))
                            .route("/delete", delete().to(PostHandler::delete))
                            .route("/like", get().to(PostHandler::like))
                            .route("/cancel_like", get().to(PostHandler::cancel_like))
                            .route("/hate", get().to(PostHandler::hate))
                            .route("/cancel_hate", get().to(PostHandler::cancel_hate))
                            .route("/get_post", get().to(PostHandler::get_one))
                            .route("/my_post", post().to(PostHandler::mine))
                            .route("/comment", post().to(PostHandler::comment))
                    ),
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
