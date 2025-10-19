//! Transaction模块 - 交易管理

pub mod entity;
pub mod repository;

// 重导出
pub use entity::{Transaction, TransactionItemType, TransactionStatus};
pub use repository::TransactionRepository;

// ID类型定义
use shared::Id;
pub type TransactionId = Id<Transaction>;