//! PostgreSQL 连接池

use sqlx::postgres::{PgPoolOptions, PgPool as SqlxPgPool};
use std::time::Duration;

pub type PgPool = SqlxPgPool;

/// 创建数据库连接池
pub async fn create_pool(
    database_url: &str,
    max_connections: u32,
    min_connections: u32,
    connect_timeout: u64,
    acquire_timeout: u64,
) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(max_connections)
        .min_connections(min_connections)
        .acquire_timeout(Duration::from_secs(acquire_timeout))
        .idle_timeout(Some(Duration::from_secs(600)))
        .test_before_acquire(true)
        .connect(database_url)
        .await
}
