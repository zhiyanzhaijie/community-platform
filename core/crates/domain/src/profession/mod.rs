//! 职业模块 - 职业分类和标准管理

pub mod entity;
pub mod repository;
pub mod value_objects;

// 重导出
pub use entity::{ProfessionStandardEntity, ProfessionStandardHistory, StandardAction};
pub use repository::ProfessionStandardRepository;
pub use value_objects::{ProfessionStandard, ProfessionType};

// ID类型定义
use shared::Id;
pub type ProfessionStandardId = Id<ProfessionStandardEntity>;
