//! Knowledge Repository trait

use async_trait::async_trait;

use crate::member::MemberId;
use shared::Result;

use super::{Knowledge, KnowledgeId};

/// 知识仓储接口
#[async_trait]
pub trait KnowledgeRepository: Send + Sync {
    /// 保存知识
    async fn save(&self, knowledge: &Knowledge) -> Result<()>;

    /// 根据 ID 查找
    async fn find_by_id(&self, id: KnowledgeId) -> Result<Option<Knowledge>>;

    /// 根据所有者查找（分页）
    async fn find_by_owner(
        &self,
        owner_id: MemberId,
        page: i64,
        page_size: i64,
    ) -> Result<Vec<Knowledge>>;

    /// 列出所有知识（分页）
    async fn find_all(&self, page: i64, page_size: i64) -> Result<Vec<Knowledge>>;

    /// 更新知识
    async fn update(&self, knowledge: &Knowledge) -> Result<()>;

    /// 删除知识
    async fn delete(&self, id: KnowledgeId) -> Result<()>;

    /// 统计总数
    async fn count_all(&self) -> Result<i64>;
}
