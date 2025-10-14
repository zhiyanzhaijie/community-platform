//! Tool Repository PostgreSQL 实现

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use domain::{
    member::MemberId,
    tool::{Currency, Money, Tool, ToolId, ToolRepository, ToolStatus},
};
use shared::{AppError, Result};
use sqlx::{FromRow, PgPool};
use std::convert::TryFrom;
use tracing::instrument;
use uuid::Uuid;

/// PostgreSQL Tool Repository
pub struct PostgresToolRepository {
    pool: PgPool,
}

impl PostgresToolRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

/// 数据库行结构
#[derive(Debug, Clone, FromRow)]
struct ToolRow {
    id: Uuid,
    owner_id: Uuid,
    name: String,
    description: Option<String>,
    category: String,
    price_amount: i64,
    price_currency: String,
    status: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

/// Row -> Domain 转换
impl TryFrom<ToolRow> for Tool {
    type Error = AppError;

    fn try_from(row: ToolRow) -> Result<Self> {
        let currency = match row.price_currency.as_str() {
            "USD" => Currency::USD,
            _ => Currency::CNY,
        };
        
        let price = Money::new(row.price_amount, currency)?;
        
        let status = match row.status.as_str() {
            "rented" => ToolStatus::Rented,
            "unavailable" => ToolStatus::Unavailable,
            _ => ToolStatus::Available,
        };

        Ok(Tool {
            id: ToolId::from_uuid(row.id),
            owner_id: MemberId::from_uuid(row.owner_id),
            name: row.name,
            description: row.description,
            category: row.category,
            price,
            status,
            created_at: row.created_at,
            updated_at: row.updated_at,
        })
    }
}

#[async_trait]
impl ToolRepository for PostgresToolRepository {
    #[instrument(name = "save_tool", skip(self, tool))]
    async fn save(&self, tool: &Tool) -> Result<()> {
        let currency_str = match tool.price.currency {
            Currency::CNY => "CNY",
            Currency::USD => "USD",
        };

        sqlx::query!(
            r#"
            INSERT INTO tools (id, owner_id, name, description, category, price_amount, price_currency, status, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            "#,
            tool.id.value(),
            tool.owner_id.value(),
            tool.name,
            tool.description,
            tool.category,
            tool.price.amount,
            currency_str,
            tool.status.to_string(),
            tool.created_at,
            tool.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::internal(format!("保存工具失败: {}", e)))?;

        Ok(())
    }

    #[instrument(name = "find_tool_by_id", skip(self))]
    async fn find_by_id(&self, id: ToolId) -> Result<Option<Tool>> {
        sqlx::query_as::<_, ToolRow>(
            "SELECT id, owner_id, name, description, category, price_amount, price_currency, status, created_at, updated_at 
             FROM tools WHERE id = $1",
        )
        .bind(id.value())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::internal(format!("查询失败: {}", e)))?
        .map(Tool::try_from)
        .transpose()
    }

    #[instrument(name = "find_tools_by_owner", skip(self))]
    async fn find_by_owner(
        &self,
        owner_id: MemberId,
        page: i64,
        page_size: i64,
    ) -> Result<Vec<Tool>> {
        let offset = (page - 1) * page_size;

        sqlx::query_as::<_, ToolRow>(
            "SELECT id, owner_id, name, description, category, price_amount, price_currency, status, created_at, updated_at 
             FROM tools WHERE owner_id = $1
             ORDER BY created_at DESC
             LIMIT $2 OFFSET $3",
        )
        .bind(owner_id.value())
        .bind(page_size)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::internal(format!("查询失败: {}", e)))?
        .into_iter()
        .map(Tool::try_from)
        .collect()
    }

    #[instrument(name = "find_available_tools", skip(self))]
    async fn find_available(&self, page: i64, page_size: i64) -> Result<Vec<Tool>> {
        let offset = (page - 1) * page_size;

        sqlx::query_as::<_, ToolRow>(
            "SELECT id, owner_id, name, description, category, price_amount, price_currency, status, created_at, updated_at 
             FROM tools WHERE status = 'available'
             ORDER BY created_at DESC
             LIMIT $1 OFFSET $2",
        )
        .bind(page_size)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::internal(format!("查询失败: {}", e)))?
        .into_iter()
        .map(Tool::try_from)
        .collect()
    }

    #[instrument(name = "update_tool", skip(self, tool))]
    async fn update(&self, tool: &Tool) -> Result<()> {
        let currency_str = match tool.price.currency {
            Currency::CNY => "CNY",
            Currency::USD => "USD",
        };

        sqlx::query!(
            r#"
            UPDATE tools
            SET name = $2, description = $3, category = $4, price_amount = $5, 
                price_currency = $6, status = $7, updated_at = $8
            WHERE id = $1
            "#,
            tool.id.value(),
            tool.name,
            tool.description,
            tool.category,
            tool.price.amount,
            currency_str,
            tool.status.to_string(),
            tool.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::internal(format!("更新工具失败: {}", e)))?;

        Ok(())
    }

    #[instrument(name = "delete_tool", skip(self))]
    async fn delete(&self, id: ToolId) -> Result<()> {
        sqlx::query!("DELETE FROM tools WHERE id = $1", id.value())
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::internal(format!("删除失败: {}", e)))?;

        Ok(())
    }

    #[instrument(name = "count_tools", skip(self))]
    async fn count(&self) -> Result<i64> {
        let result = sqlx::query!("SELECT COUNT(*) as count FROM tools")
            .fetch_one(&self.pool)
            .await
            .map_err(|e| AppError::internal(format!("统计失败: {}", e)))?;

        Ok(result.count.unwrap_or(0))
    }
}
