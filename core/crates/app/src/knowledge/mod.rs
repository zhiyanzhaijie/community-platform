//! 知识用例

pub mod commands;
pub mod queries;

// 导出命令
pub use commands::{
    create_knowledge, delete_knowledge, update_knowledge, CreateKnowledgeInput, UpdateKnowledgeInput,
};

// 导出查询
pub use queries::{
    get_knowledge, list_knowledge, list_knowledge_by_owner, count_knowledge,
};
