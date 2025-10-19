//! Member Repository PostgreSQL 实现

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use domain::member::{Email, Member, MemberId, MemberRepository, Username, UserRole};
use domain::profession::ProfessionType;
use shared::{AppError, Result};
use sqlx::{FromRow, PgPool};
use std::convert::TryFrom;
use tracing::instrument;
use uuid::Uuid;

/// PostgreSQL Member Repository
pub struct PostgresMemberRepository {
    pool: PgPool,
}

impl PostgresMemberRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

/// 数据库行结构
#[derive(Debug, Clone, FromRow)]
struct MemberRow {
    id: Uuid,
    email: String,
    username: String,
    password_hash: String,
    status: String,
    role: String,
    managed_professions: Option<serde_json::Value>, // JSON
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

/// Row -> Domain 转换
impl TryFrom<MemberRow> for Member {
    type Error = AppError;

    fn try_from(row: MemberRow) -> Result<Self> {
        // 解析managed_professions JSON
        let managed_professions = if let Some(json_value) = row.managed_professions {
            if json_value.is_null() {
                Vec::new()
            } else {
                serde_json::from_value(json_value)
                    .map_err(|e| AppError::internal(format!("解析职业列表失败: {}", e)))?
            }
        } else {
            Vec::new()
        };

        Ok(Member {
            id: MemberId::from_uuid(row.id),
            email: Email::new(row.email)
                .map_err(|e| AppError::internal(format!("数据库中的邮箱格式无效: {}", e)))?,
            username: Username::new(row.username)
                .map_err(|e| AppError::internal(format!("数据库中的用户名格式无效: {}", e)))?,
            password_hash: row.password_hash,
            status: row.status.parse()?,
            role: row.role.parse()?,
            managed_professions,
            created_at: row.created_at,
            updated_at: row.updated_at,
        })
    }
}

#[async_trait]
impl MemberRepository for PostgresMemberRepository {
    #[instrument(name = "save_member", skip(self, member))]
    async fn save(&self, member: &Member) -> Result<()> {
        let managed_professions_json = if member.managed_professions.is_empty() {
            None
        } else {
            Some(serde_json::to_value(&member.managed_professions)
                .map_err(|e| AppError::internal(format!("序列化职业列表失败: {}", e)))?)
        };

        sqlx::query!(
            r#"
            INSERT INTO members (id, email, username, password_hash, status, role, managed_professions, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#,
            member.id.value(),
            member.email.value(),
            member.username.value(),
            member.password_hash,
            member.status.to_string(),
            member.role.to_string(),
            managed_professions_json,
            member.created_at,
            member.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::internal(format!("保存会员失败: {}", e)))?;

        Ok(())
    }

    #[instrument(name = "find_member_by_id", skip(self))]
    async fn find_by_id(&self, id: MemberId) -> Result<Option<Member>> {
        sqlx::query_as::<_, MemberRow>(
            "SELECT id, email, username, password_hash, status, role, managed_professions, created_at, updated_at 
             FROM members WHERE id = $1",
        )
        .bind(id.value())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::internal(format!("查询失败: {}", e)))?
        .map(Member::try_from)
        .transpose()
    }

    #[instrument(name = "find_member_by_email", skip(self))]
    async fn find_by_email(&self, email: &Email) -> Result<Option<Member>> {
        sqlx::query_as::<_, MemberRow>(
            "SELECT id, email, username, password_hash, status, role, managed_professions, created_at, updated_at 
             FROM members WHERE email = $1",
        )
        .bind(email.value())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::internal(format!("查询失败: {}", e)))?
        .map(Member::try_from)
        .transpose()
    }

    #[instrument(name = "find_member_by_username", skip(self))]
    async fn find_by_username(&self, username: &Username) -> Result<Option<Member>> {
        sqlx::query_as::<_, MemberRow>(
            "SELECT id, email, username, password_hash, status, role, managed_professions, created_at, updated_at 
             FROM members WHERE username = $1",
        )
        .bind(username.value())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::internal(format!("查询失败: {}", e)))?
        .map(Member::try_from)
        .transpose()
    }

    #[instrument(name = "update_member", skip(self, member))]
    async fn update(&self, member: &Member) -> Result<()> {
        let managed_professions_json = if member.managed_professions.is_empty() {
            None
        } else {
            Some(serde_json::to_value(&member.managed_professions)
                .map_err(|e| AppError::internal(format!("序列化职业列表失败: {}", e)))?)
        };

        sqlx::query!(
            r#"
            UPDATE members
            SET email = $2, username = $3, password_hash = $4, status = $5, role = $6, managed_professions = $7, updated_at = $8
            WHERE id = $1
            "#,
            member.id.value(),
            member.email.value(),
            member.username.value(),
            member.password_hash,
            member.status.to_string(),
            member.role.to_string(),
            managed_professions_json,
            member.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::internal(format!("更新会员失败: {}", e)))?;

        Ok(())
    }

    #[instrument(name = "delete_member", skip(self))]
    async fn delete(&self, id: MemberId) -> Result<()> {
        sqlx::query!("DELETE FROM members WHERE id = $1", id.value())
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::internal(format!("删除失败: {}", e)))?;

        Ok(())
    }
}
