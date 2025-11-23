//! Knowledge Repository PostgreSQL 实现

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use domain::{
    member::MemberId,
    knowledge::{Knowledge, KnowledgeForm, KnowledgeId, KnowledgeRepository},
};
use serde_json::Value;
use shared::{AppError, Result};
use sqlx::{FromRow, PgPool};
use std::convert::TryFrom;
use std::str::FromStr;
use tracing::instrument;
use uuid::Uuid;

/// PostgreSQL Knowledge Repository
pub struct PostgresKnowledgeRepository {
    pool: PgPool,
}

impl PostgresKnowledgeRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

/// 数据库行结构
#[derive(Debug, Clone, FromRow)]
struct KnowledgeRow {
    id: Uuid,
    owner_id: Uuid,
    title: String,
    summary: Option<String>,
    category: String,
    form: String,
    content: Value,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

/// Row -> Domain 转换
impl TryFrom<KnowledgeRow> for Knowledge {
    type Error = AppError;

    fn try_from(row: KnowledgeRow) -> Result<Self> {
        let form = KnowledgeForm::from_str(row.form.as_str())?;

        Ok(Knowledge {
            id: KnowledgeId::from_uuid(row.id),
            owner_id: MemberId::from_uuid(row.owner_id),
            title: row.title,
            summary: row.summary,
            category: row.category,
            form,
            content: row.content,
            created_at: row.created_at,
            updated_at: row.updated_at,
        })
    }
}

#[async_trait]
impl KnowledgeRepository for PostgresKnowledgeRepository {
    #[instrument(name = "save_knowledge", skip(self, knowledge))]
    async fn save(&self, knowledge: &Knowledge) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO knowledge_items (id, owner_id, title, summary, category, form, content, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#,
            knowledge.id.value(),
            knowledge.owner_id.value(),
            knowledge.title,
            knowledge.summary,
            knowledge.category,
            knowledge.form.to_string(),
            knowledge.content,
            knowledge.created_at,
            knowledge.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::internal(format!("保存知识失败: {}", e)))?;

        Ok(())
    }

    #[instrument(name = "find_knowledge_by_id", skip(self))]
    async fn find_by_id(&self, id: KnowledgeId) -> Result<Option<Knowledge>> {
        sqlx::query_as::<_, KnowledgeRow>(
            "SELECT id, owner_id, title, summary, category, form, content, created_at, updated_at \
             FROM knowledge_items WHERE id = $1",
        )
        .bind(id.value())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::internal(format!("查询失败: {}", e)))?
        .map(Knowledge::try_from)
        .transpose()
    }

    #[instrument(name = "find_knowledge_by_owner", skip(self))]
    async fn find_by_owner(
        &self,
        owner_id: MemberId,
        page: i64,
        page_size: i64,
    ) -> Result<Vec<Knowledge>> {
        let offset = (page - 1) * page_size;

        sqlx::query_as::<_, KnowledgeRow>(
            "SELECT id, owner_id, title, summary, category, form, content, created_at, updated_at \
             FROM knowledge_items WHERE owner_id = $1 \
             ORDER BY created_at DESC \
             LIMIT $2 OFFSET $3",
        )
        .bind(owner_id.value())
        .bind(page_size)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::internal(format!("查询失败: {}", e)))?
        .into_iter()
        .map(Knowledge::try_from)
        .collect()
    }

    #[instrument(name = "find_all_knowledge", skip(self))]
    async fn find_all(&self, page: i64, page_size: i64) -> Result<Vec<Knowledge>> {
        let offset = (page - 1) * page_size;

        sqlx::query_as::<_, KnowledgeRow>(
            "SELECT id, owner_id, title, summary, category, form, content, created_at, updated_at \
             FROM knowledge_items \
             ORDER BY created_at DESC \
             LIMIT $1 OFFSET $2",
        )
        .bind(page_size)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::internal(format!("查询失败: {}", e)))?
        .into_iter()
        .map(Knowledge::try_from)
        .collect()
    }

    #[instrument(name = "update_knowledge", skip(self, knowledge))]
    async fn update(&self, knowledge: &Knowledge) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE knowledge_items
            SET title = $2, summary = $3, category = $4, form = $5, content = $6, updated_at = $7
            WHERE id = $1
            "#,
            knowledge.id.value(),
            knowledge.title,
            knowledge.summary,
            knowledge.category,
            knowledge.form.to_string(),
            knowledge.content,
            knowledge.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::internal(format!("更新知识失败: {}", e)))?;

        Ok(())
    }

    #[instrument(name = "delete_knowledge", skip(self))]
    async fn delete(&self, id: KnowledgeId) -> Result<()> {
        sqlx::query!("DELETE FROM knowledge_items WHERE id = $1", id.value())
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::internal(format!("删除失败: {}", e)))?;

        Ok(())
    }

    #[instrument(name = "count_knowledge", skip(self))]
    async fn count_all(&self) -> Result<i64> {
        let result = sqlx::query!("SELECT COUNT(*) as count FROM knowledge_items")
            .fetch_one(&self.pool)
            .await
            .map_err(|e| AppError::internal(format!("统计失败: {}", e)))?;

        Ok(result.count.unwrap_or(0))
    }
}
