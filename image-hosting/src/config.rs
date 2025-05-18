use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Serialize)]
//
// DATABASE_URL='postgres://image_hosting:image_hosting@127.0.0.1:5432/image_hosting'
// DATABASE_MAX_CONNS=5
pub struct Config {
    pub b2_bucket_name: String,
    pub b2_key_id: String,
    pub b2_application_key: String,
    pub b2_endpoint: String,
    pub url_prefix: String,
    pub database_url: String,
    pub database_max_conns: u32,
}

impl Config {
    pub fn from_env() -> Result<Self, ::config::ConfigError> {
        ::config::Config::builder()
            .add_source(::config::Environment::default())
            .build()?
            .try_deserialize()
    }
}

lazy_static! {
    pub static ref CFG: Arc<Config> = {
        dotenv::dotenv().ok();
        Arc::new(Config::from_env().unwrap())
    };
}
