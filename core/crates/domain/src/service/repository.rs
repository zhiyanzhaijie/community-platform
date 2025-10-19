//! Service Repository接口

use super::{Service, ServiceId};
use crate::member::MemberId;
use crate::profession::ProfessionType;
use async_trait::async_trait;
use shared::Result;

/// 服务Repository trait
#[async_trait]
pub trait ServiceRepository: Send + Sync {
    /// 保存服务
    async fn save(&self, service: &Service) -> Result<()>;

    /// 根据ID查找服务
    async fn find_by_id(&self, id: &ServiceId) -> Result<Option<Service>>;

    /// 根据提供者ID查找服务
    async fn find_by_provider_id(&self, provider_id: &MemberId) -> Result<Vec<Service>>;

    /// 根据职业类型查找可用服务
    async fn find_available_by_profession(
        &self,
        profession_type: ProfessionType,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<Vec<Service>>;

    /// 查找所有可用服务（分页）
    async fn find_available_services(
        &self,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<Vec<Service>>;

    /// 根据关键词搜索服务
    async fn search_services(
        &self,
        keyword: &str,
        profession_type: Option<ProfessionType>,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<Vec<Service>>;

    /// 删除服务
    async fn delete(&self, id: &ServiceId) -> Result<()>;

    /// 统计服务数量
    async fn count_by_provider(&self, provider_id: &MemberId) -> Result<u64>;

    /// 统计可用服务数量
    async fn count_available_services(&self) -> Result<u64>;
}