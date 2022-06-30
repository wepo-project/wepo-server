mod config;
mod db;
mod dto;
mod errors;
mod handlers;
mod models;

use crate::config::WepoConfig;
use ::config::Config;
use actix_web::{web, App, HttpServer};
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

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(web::scope("/v1").service(handlers::user::register_user))
    })
    .bind(config.server_addr.clone())?
    .run();

    info!("Server running at http://{}/", config.server_addr);

    server.await
}

pub fn init_logger () {
	use chrono::Local;
	use std::io::Write;

	let env = env_logger::Env::default()
		.filter_or(env_logger::DEFAULT_FILTER_ENV, "info");
	// 设置打印日志的格式
	env_logger::Builder::from_env(env)
		.format(|buf, record|{
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