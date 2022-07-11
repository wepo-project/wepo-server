mod base;
mod config;
mod data_models;
mod db;
mod errors;
mod models;
mod utils;

use crate::config::WepoConfig;
use crate::{
    models::post::handler as PostHandler,
    models::user::handler as UserHandler,
};
use ::config::Config;
use actix_cors::Cors;
use actix_redis::RedisActor;
use actix_web::{http, web::{self, get, post, delete}, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
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
        let auth = HttpAuthentication::bearer(models::user::handler::bearer_handle);
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
                            .route("/login", get().to(UserHandler::token_login))
                            .wrap(HttpAuthentication::bearer(models::user::handler::token_addon_middleware)),
                    )
                    .service(
                        web::scope("/user")
                            .route("/add_user", post().to(UserHandler::register_user))
                            .route("/login", post().to(UserHandler::user_login)),
                    )
                    .service(
                        web::scope("/post")
                            .route("/add_post", post().to(PostHandler::add_post))
                            .route("/del_post", delete().to(PostHandler::delete_post))
                            .route("/like", get().to(PostHandler::post_like))
                            .route("/unlike", get().to(PostHandler::post_unlike))
                            .route("/get_post", get().to(PostHandler::get_post))
                            .route("/my_post", post().to(PostHandler::my_post))
                            .wrap(auth),
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
