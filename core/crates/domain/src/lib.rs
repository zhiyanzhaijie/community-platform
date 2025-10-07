//! 领域层
//! 包含核心业务逻辑和领域模型

pub mod member;
pub mod tool;

// 重导出共享类型
pub use shared::{AppError, Result};
