//! Knowledge DTOs

use domain::knowledge::Knowledge;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;

#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub struct CreateKnowledgeRequest {
    pub title: String,
    pub summary: Option<String>,
    pub category: String,
    /// 知识形式: "text" 或 "graph"
    pub form: String,
    /// 知识内容，结构由 form 决定
    pub content: Value,
}

#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub struct UpdateKnowledgeRequest {
    pub title: Option<String>,
    /// Some(Some(x)) = 更新为 x，Some(None) = 置空，None = 不变
    pub summary: Option<Option<String>>,
    pub category: Option<String>,
    pub form: Option<String>,
    pub content: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct KnowledgeDto {
    pub id: String,
    pub owner_id: String,
    pub title: String,
    pub summary: Option<String>,
    pub category: String,
    pub form: String,
    pub content: Value,
    pub created_at: String,
    pub updated_at: String,
}

impl From<&Knowledge> for KnowledgeDto {
    fn from(k: &Knowledge) -> Self {
        Self {
            id: k.id.to_string(),
            owner_id: k.owner_id.to_string(),
            title: k.title.clone(),
            summary: k.summary.clone(),
            category: k.category.clone(),
            form: k.form.to_string(),
            content: k.content.clone(),
            created_at: k.created_at.to_rfc3339(),
            updated_at: k.updated_at.to_rfc3339(),
        }
    }
}
