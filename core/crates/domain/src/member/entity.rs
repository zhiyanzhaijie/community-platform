//! Member 实体

use super::{Email, MemberId, MemberStatus, Username, UserRole};
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
    pub role: UserRole,
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
            role: UserRole::default(), // 默认为普通用户
            created_at: now,
            updated_at: now,
        }
    }

    /// 修改状态（私有方法）
    fn change_status(&mut self, new_status: MemberStatus) {
        self.status = new_status;
        self.updated_at = Utc::now();
    }

    /// 激活会员
    pub fn activate(&mut self) {
        self.change_status(MemberStatus::Active);
    }

    /// 停用会员
    pub fn deactivate(&mut self) {
        self.change_status(MemberStatus::Inactive);
    }

    /// 封禁会员
    pub fn ban(&mut self) {
        self.change_status(MemberStatus::Banned);
    }

    /// 检查是否激活
    pub fn is_active(&self) -> bool {
        self.status == MemberStatus::Active
    }

    /// 检查是否是决策者
    pub fn is_decider(&self) -> bool {
        self.role == UserRole::Decider
    }

    /// 检查是否是管理员
    pub fn is_admin(&self) -> bool {
        self.role == UserRole::Admin
    }

    /// 提升为决策者（只有管理员可以操作）
    pub fn promote_to_decider(&mut self) {
        self.role = UserRole::Decider;
        self.updated_at = Utc::now();
    }

    /// 提升为管理员
    pub fn promote_to_admin(&mut self) {
        self.role = UserRole::Admin;
        self.updated_at = Utc::now();
    }

    /// 降级为普通用户
    pub fn demote_to_regular(&mut self) {
        self.role = UserRole::Regular;
        self.updated_at = Utc::now();
    }
}
