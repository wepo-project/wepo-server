use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct WepoConfig {
    pub server_addr: String,
    pub redis_addr: String,
    pub pg: deadpool_postgres::Config,
}