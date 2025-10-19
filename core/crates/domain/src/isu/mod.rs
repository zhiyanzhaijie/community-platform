//! ISU模块 - 国际标准单位系统

pub mod entity;
pub mod repository;
pub mod value_objects;

// 重导出
pub use entity::{ISUAccount, ISUTransaction, ISUTransactionType};
pub use repository::ISUAccountRepository;
pub use value_objects::{ISU, ISURate};

// ID类型定义
use shared::Id;
pub type ISUAccountId = Id<ISUAccount>;