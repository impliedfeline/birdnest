use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CacheConfig {
    pub ndz_center_x: f64,
    pub ndz_center_y: f64,
    pub ndz_radius: f64,
    pub server_url: String,
    pub violation_ttl_secs: u64,
    pub update_interval_secs: u64,
}

impl CacheConfig {
    pub fn new() -> Result<Self, ConfigError> {
        let config = Config::builder()
            .add_source(File::with_name("config/cache"))
            .build()?;
        config.try_deserialize()
    }
}
