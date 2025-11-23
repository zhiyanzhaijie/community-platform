//! 领域层
//! 包含核心业务逻辑和领域模型

pub mod knowledge;
pub mod member;

// 重导出共享类型
pub use shared::{AppError, Result};
