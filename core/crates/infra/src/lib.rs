//! 基础设施层
//! 提供数据持久化、日志等基础设施实现

pub mod persistence;
pub mod tracing_setup;

pub use persistence::postgres::{create_pool, PgPool, PostgresMemberRepository};
pub use tracing_setup::init_tracing;
