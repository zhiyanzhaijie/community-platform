//! 应用状态管理

use std::sync::Arc;

use domain::{knowledge::KnowledgeRepository, member::MemberRepository};
use shared::AppConfig;

#[derive(Clone)]
pub struct AppState {
    pub member_repo: Arc<dyn MemberRepository>,
    pub knowledge_repo: Arc<dyn KnowledgeRepository>,
    pub password_hasher: Arc<dyn infra::PasswordHasher>,
    pub config: Arc<AppConfig>,
}

impl AppState {
    pub fn new(
        member_repo: Arc<dyn MemberRepository>,
        knowledge_repo: Arc<dyn KnowledgeRepository>,
        password_hasher: Arc<dyn infra::PasswordHasher>,
        config: Arc<AppConfig>,
    ) -> Self {
        Self {
            member_repo,
            knowledge_repo,
            password_hasher,
            config,
        }
    }
}
