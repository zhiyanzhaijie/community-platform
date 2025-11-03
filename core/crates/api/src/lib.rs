//! API 接口层
//! 
//! 这个 crate 分为两部分:
//! - `dto`: 数据传输对象,轻量级,始终可用
//! - HTTP 服务层: 需要 `http-server` feature 才能使用

// ========== DTO 层 (始终可用) ==========
pub mod dto;

// ========== HTTP 服务层 (需要 http-server feature) ==========
#[cfg(feature = "http-server")]
pub mod middleware;

#[cfg(feature = "http-server")]
pub mod openapi;

#[cfg(feature = "http-server")]
pub mod routes;

#[cfg(feature = "http-server")]
pub mod state;

#[cfg(feature = "http-server")]
pub mod v1;

// 导出常用类型
#[cfg(feature = "http-server")]
pub use state::AppState;
