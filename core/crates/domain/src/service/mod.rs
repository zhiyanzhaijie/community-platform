//! Service模块 - 服务发布和管理

pub mod entity;
pub mod repository;

// 重导出
pub use entity::{Service, ServiceStatus};
pub use repository::ServiceRepository;

// ID类型定义
use shared::Id;
pub type ServiceId = Id<Service>;