//! Tool Repository trait

use super::{Tool, ToolId};
use crate::member::MemberId;
use async_trait::async_trait;
use shared::Result;

/// 工具仓储接口
#[async_trait]
pub trait ToolRepository: Send + Sync {
    /// 保存工具
    async fn save(&self, tool: &Tool) -> Result<()>;

    /// 根据 ID 查找
    async fn find_by_id(&self, id: ToolId) -> Result<Option<Tool>>;

    /// 根据所有者查找
    async fn find_by_owner(
        &self,
        owner_id: MemberId,
        page: i64,
        page_size: i64,
    ) -> Result<Vec<Tool>>;

    /// 查找所有可用工具（分页）
    async fn find_available(&self, page: i64, page_size: i64) -> Result<Vec<Tool>>;

    /// 更新工具
    async fn update(&self, tool: &Tool) -> Result<()>;

    /// 删除工具
    async fn delete(&self, id: ToolId) -> Result<()>;

    /// 统计总数
    async fn count(&self) -> Result<i64>;
}
