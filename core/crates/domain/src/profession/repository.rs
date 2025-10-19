//! 职业标准Repository接口

use super::{ProfessionStandardEntity, ProfessionStandardHistory, ProfessionStandardId, ProfessionType};
use crate::member::MemberId;
use async_trait::async_trait;
use shared::Result;

/// 职业标准Repository trait
#[async_trait]
pub trait ProfessionStandardRepository: Send + Sync {
    /// 保存职业标准
    async fn save(&self, standard: &ProfessionStandardEntity) -> Result<()>;

    /// 根据ID查找职业标准
    async fn find_by_id(&self, id: &ProfessionStandardId) -> Result<Option<ProfessionStandardEntity>>;

    /// 根据职业类型查找当前活跃的标准
    async fn find_active_by_profession(&self, profession_type: ProfessionType) -> Result<Option<ProfessionStandardEntity>>;

    /// 根据职业类型查找所有标准（包括历史）
    async fn find_all_by_profession(&self, profession_type: ProfessionType) -> Result<Vec<ProfessionStandardEntity>>;

    /// 查找决策者管理的所有职业标准
    async fn find_by_manager(&self, manager_id: &MemberId) -> Result<Vec<ProfessionStandardEntity>>;

    /// 更新职业标准
    async fn update(&self, standard: &ProfessionStandardEntity) -> Result<()>;

    /// 删除职业标准
    async fn delete(&self, id: &ProfessionStandardId) -> Result<()>;

    /// 保存变更历史
    async fn save_history(&self, history: &ProfessionStandardHistory) -> Result<()>;

    /// 获取职业标准的变更历史
    async fn get_history(&self, standard_id: &ProfessionStandardId) -> Result<Vec<ProfessionStandardHistory>>;

    /// 获取所有活跃的职业标准
    async fn find_all_active(&self) -> Result<Vec<ProfessionStandardEntity>>;
}