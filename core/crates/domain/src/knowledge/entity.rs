//! Knowledge 实体

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::member::MemberId;

use super::{KnowledgeForm, KnowledgeId};

/// 知识聚合根
///
/// MVP 版本：
/// - 以范畴(category) 对知识进行分类（哲学上的范畴）
/// - 以形式(form) 区分不同内容表现（文本 / 图形）
/// - content 使用 JSON 存储，具体结构由 form 决定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Knowledge {
    pub id: KnowledgeId,
    pub owner_id: MemberId,
    pub title: String,
    pub summary: Option<String>,
    pub category: String,
    pub form: KnowledgeForm,
    pub content: Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Knowledge {
    /// 创建新的知识条目
    pub fn new(
        owner_id: MemberId,
        title: String,
        summary: Option<String>,
        category: String,
        form: KnowledgeForm,
        content: Value,
    ) -> Self {
        let now = Utc::now();

        Self {
            id: KnowledgeId::new(),
            owner_id,
            title: title.trim().to_string(),
            summary: summary.map(|s| s.trim().to_string()).filter(|s| !s.is_empty()),
            category: category.trim().to_string(),
            form,
            content,
            created_at: now,
            updated_at: now,
        }
    }

    /// 更新知识内容
    pub fn update(
        &mut self,
        title: Option<String>,
        summary: Option<Option<String>>,
        category: Option<String>,
        form: Option<KnowledgeForm>,
        content: Option<Value>,
    ) {
        if let Some(title) = title {
            let t = title.trim();
            if !t.is_empty() {
                self.title = t.to_string();
            }
        }

        if let Some(summary_opt) = summary {
            self.summary = summary_opt.map(|s| s.trim().to_string()).filter(|s| !s.is_empty());
        }

        if let Some(category) = category {
            let c = category.trim();
            if !c.is_empty() {
                self.category = c.to_string();
            }
        }

        if let Some(form) = form {
            self.form = form;
        }

        if let Some(content) = content {
            self.content = content;
        }

        self.updated_at = Utc::now();
    }
}
