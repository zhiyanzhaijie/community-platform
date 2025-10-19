//! 领域层
//! 包含核心业务逻辑和领域模型

pub mod isu;
pub mod member;
pub mod profession;
pub mod service;
pub mod tool;
pub mod transaction;

// 重导出共享类型
pub use shared::{AppError, Result};
