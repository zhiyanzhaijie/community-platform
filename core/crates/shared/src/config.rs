//! 配置管理模块

use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub jwt: JwtConfig,
    pub log: LogConfig,
    pub pagination: PaginationConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout: u64,
    pub acquire_timeout: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct JwtConfig {
    pub secret: String,
    pub expires_in: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LogConfig {
    pub level: String,
    pub format: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PaginationConfig {
    pub default_page_size: i64,
    pub max_page_size: i64,
}

impl AppConfig {
    /// 加载配置
    pub fn load() -> Result<Self, ConfigError> {
        dotenvy::dotenv().ok();
        let run_env = env::var("RUN_ENV").unwrap_or_else(|_| "local".into());

        let config = Config::builder()
            .add_source(File::with_name("config/default"))
            .add_source(File::with_name(&format!("config/{}", run_env)).required(false))
            .add_source(Environment::with_prefix("APP").separator("__"))
            .build()?;

        config.try_deserialize()
    }
}
