//! # Shared Crate
//! 提供配置管理、统一错误类型、常量定义等共享功能

pub mod config;
pub mod constants;
pub mod error;
pub mod types;

// 重导出常用类型
pub use config::AppConfig;
pub use error::{AppError, Result};
pub use types::Id;
