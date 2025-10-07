//! Member 实体

use super::{Email, MemberId, MemberStatus, Username};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 会员聚合根
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Member {
    pub id: MemberId,
    pub email: Email,
    pub username: Username,
    pub password_hash: String,
    pub status: MemberStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Member {
    /// 创建新会员
    pub fn new(email: Email, username: Username, password_hash: String) -> Self {
        let now = Utc::now();
        Self {
            id: MemberId::new(),
            email,
            username,
            password_hash,
            status: MemberStatus::default(),
            created_at: now,
            updated_at: now,
        }
    }

    /// 激活会员
    pub fn activate(&mut self) {
        self.status = MemberStatus::Active;
        self.updated_at = Utc::now();
    }

    /// 停用会员
    pub fn deactivate(&mut self) {
        self.status = MemberStatus::Inactive;
        self.updated_at = Utc::now();
    }

    /// 封禁会员
    pub fn ban(&mut self) {
        self.status = MemberStatus::Banned;
        self.updated_at = Utc::now();
    }

    /// 检查是否激活
    pub fn is_active(&self) -> bool {
        self.status == MemberStatus::Active
    }
}
