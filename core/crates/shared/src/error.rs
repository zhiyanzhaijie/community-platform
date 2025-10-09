//! 统一错误类型

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
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

// 错误响应结构
#[derive(Serialize)]
struct ErrorResponse {
    success: bool,
    message: String,
}

// 实现 IntoResponse
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Validation(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "未授权".to_string()),
            AppError::Forbidden => (StatusCode::FORBIDDEN, "权限不足".to_string()),
            AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::Config(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("配置错误: {}", e),
            ),
        };

        let body = Json(ErrorResponse {
            success: false,
            message,
        });
        
        (status, body).into_response()
    }
}
