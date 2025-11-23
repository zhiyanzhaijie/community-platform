//! 知识相关值对象

use serde::{Deserialize, Serialize};
use std::str::FromStr;

use shared::AppError;

/// 知识形式（内容表示方式）
/// - Text: 富文本/文档式内容（Notion 类似）
/// - Graph: 图形/关系图内容（Excalidraw 类 JSON）
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum KnowledgeForm {
    Text,
    Graph,
}

impl KnowledgeForm {
    pub fn as_str(&self) -> &'static str {
        match self {
            KnowledgeForm::Text => "text",
            KnowledgeForm::Graph => "graph",
        }
    }
}

impl std::fmt::Display for KnowledgeForm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for KnowledgeForm {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "text" => Ok(KnowledgeForm::Text),
            "graph" => Ok(KnowledgeForm::Graph),
            other => Err(AppError::validation(format!("未知的知识形式: {}", other))),
        }
    }
}
