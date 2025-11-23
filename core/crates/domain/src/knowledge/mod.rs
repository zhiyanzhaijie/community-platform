//! 知识聚合根

mod entity;
mod repository;
mod value_objects;

pub use entity::Knowledge;
pub use repository::KnowledgeRepository;
pub use value_objects::{KnowledgeForm};

// 类型别名
pub type KnowledgeId = shared::Id<Knowledge>;
