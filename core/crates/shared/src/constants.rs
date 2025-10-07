//! 全局常量定义
//! 
//! 只包含编译期确定且不需要运行时调整的常量

/// API 版本前缀
pub const API_VERSION_V1: &str = "/api/v1";

/// JWT HTTP Header
pub const JWT_HEADER: &str = "Authorization";
pub const JWT_BEARER_PREFIX: &str = "Bearer ";
