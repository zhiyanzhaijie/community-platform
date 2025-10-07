//! Member Repository PostgreSQL 实现

use async_trait::async_trait;
use domain::member::{Email, Member, MemberId, MemberRepository, MemberStatus, Username};
use shared::{AppError, Result};
use sqlx::PgPool;
use tracing::instrument;

/// PostgreSQL Member Repository
pub struct PostgresMemberRepository {
    pool: PgPool,
}

impl PostgresMemberRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl MemberRepository for PostgresMemberRepository {
    #[instrument(name = "save_member", skip(self, member))]
    async fn save(&self, member: &Member) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO members (id, email, username, password_hash, status, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
            member.id.value(),
            member.email.value(),
            member.username.value(),
            member.password_hash,
            member.status.to_string(),
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
        let record = sqlx::query!(
            r#"
            SELECT id, email, username, password_hash, status, created_at, updated_at
            FROM members
            WHERE id = $1
            "#,
            id.value()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::internal(format!("查询会员失败: {}", e)))?;

        Ok(record.map(|r| Member {
            id: MemberId::from_uuid(r.id),
            email: Email::new(r.email).unwrap(),
            username: Username::new(r.username).unwrap(),
            password_hash: r.password_hash,
            status: parse_member_status(&r.status),
            created_at: r.created_at,
            updated_at: r.updated_at,
        }))
    }

    #[instrument(name = "find_member_by_email", skip(self))]
    async fn find_by_email(&self, email: &Email) -> Result<Option<Member>> {
        let record = sqlx::query!(
            r#"
            SELECT id, email, username, password_hash, status, created_at, updated_at
            FROM members
            WHERE email = $1
            "#,
            email.value()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::internal(format!("查询会员失败: {}", e)))?;

        Ok(record.map(|r| Member {
            id: MemberId::from_uuid(r.id),
            email: Email::new(r.email).unwrap(),
            username: Username::new(r.username).unwrap(),
            password_hash: r.password_hash,
            status: parse_member_status(&r.status),
            created_at: r.created_at,
            updated_at: r.updated_at,
        }))
    }

    #[instrument(name = "find_member_by_username", skip(self))]
    async fn find_by_username(&self, username: &Username) -> Result<Option<Member>> {
        let record = sqlx::query!(
            r#"
            SELECT id, email, username, password_hash, status, created_at, updated_at
            FROM members
            WHERE username = $1
            "#,
            username.value()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::internal(format!("查询会员失败: {}", e)))?;

        Ok(record.map(|r| Member {
            id: MemberId::from_uuid(r.id),
            email: Email::new(r.email).unwrap(),
            username: Username::new(r.username).unwrap(),
            password_hash: r.password_hash,
            status: parse_member_status(&r.status),
            created_at: r.created_at,
            updated_at: r.updated_at,
        }))
    }

    #[instrument(name = "update_member", skip(self, member))]
    async fn update(&self, member: &Member) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE members
            SET email = $2, username = $3, password_hash = $4, status = $5, updated_at = $6
            WHERE id = $1
            "#,
            member.id.value(),
            member.email.value(),
            member.username.value(),
            member.password_hash,
            member.status.to_string(),
            member.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::internal(format!("更新会员失败: {}", e)))?;

        Ok(())
    }

    #[instrument(name = "delete_member", skip(self))]
    async fn delete(&self, id: MemberId) -> Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM members WHERE id = $1
            "#,
            id.value()
        )
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::internal(format!("删除会员失败: {}", e)))?;

        Ok(())
    }
}

/// 解析会员状态
fn parse_member_status(status: &str) -> MemberStatus {
    match status {
        "active" => MemberStatus::Active,
        "inactive" => MemberStatus::Inactive,
        "banned" => MemberStatus::Banned,
        _ => MemberStatus::Active,
    }
}
