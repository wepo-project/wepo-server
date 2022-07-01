mod config;
mod data_models;
mod db;
mod errors;
mod models;
mod utils;

use crate::config::WepoConfig;
use actix_web_httpauth::middleware::HttpAuthentication;
use ::config::Config;
use actix_redis::RedisActor;
use actix_web::{
    web,
    App, HttpServer,
};
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

    let pool = config.pg.create_pool(None, NoTls).unwrap();

    let redis_addr = RedisActor::start(config.redis_addr.clone());

    let server = HttpServer::new(move || {
        let auth = HttpAuthentication::bearer(models::user::handler::bearer_handle);
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(redis_addr.clone()))
            .service(
                web::scope("/v1")
                    .service(
                        web::scope("/user")
                        .service(models::user::handler::register_user)
                        .service(models::user::handler::user_login),
                    )
                    .service(
                        web::scope("/post")
                            .wrap(auth)
                            .service(models::post::handler::add_post),
                    )
            )
    })
    .bind(config.server_addr.clone())?
    .workers(2)
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
