//! 应用状态管理

use std::sync::Arc;

use domain::{member::MemberRepository, tool::ToolRepository};
use shared::AppConfig;

/// 全局应用状态
#[derive(Clone)]
pub struct AppState {
    pub member_repo: Arc<dyn MemberRepository>,
    pub tool_repo: Arc<dyn ToolRepository>,
    pub password_hasher: Arc<dyn infra::PasswordHasher>,
    pub config: Arc<AppConfig>,
}

impl AppState {
    /// 创建新的应用状态
    pub fn new(
        member_repo: Arc<dyn MemberRepository>,
        tool_repo: Arc<dyn ToolRepository>,
        password_hasher: Arc<dyn infra::PasswordHasher>,
        config: Arc<AppConfig>,
    ) -> Self {
        Self {
            member_repo,
            tool_repo,
            password_hasher,
            config,
        }
    }
}
