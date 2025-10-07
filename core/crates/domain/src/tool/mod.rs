//! 工具聚合根

mod entity;
mod repository;
mod value_objects;

pub use entity::Tool;
pub use repository::ToolRepository;
pub use value_objects::{Money, ToolStatus};

// 类型别名
pub type ToolId = shared::Id<Tool>;
