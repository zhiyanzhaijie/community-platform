//! 统一错误类型

use thiserror::Error;

pub type Result<T> = std::result::Result<T, AppError>;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("配置错误: {0}")]
    Config(#[from] config::ConfigError),

    #[error("内部错误: {0}")]
    Internal(String),

    #[error("未找到资源: {0}")]
    NotFound(String),

    #[error("验证错误: {0}")]
    Validation(String),

    #[error("未授权")]
    Unauthorized,

    #[error("权限不足")]
    Forbidden,
}

impl AppError {
    pub fn internal(msg: impl Into<String>) -> Self {
        Self::Internal(msg.into())
    }

    pub fn not_found(msg: impl Into<String>) -> Self {
        Self::NotFound(msg.into())
    }

    pub fn validation(msg: impl Into<String>) -> Self {
        Self::Validation(msg.into())
    }
}
