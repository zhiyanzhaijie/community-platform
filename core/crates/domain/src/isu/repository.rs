//! ISU账户Repository接口

use super::{ISU, ISUAccount, ISUAccountId, ISUTransaction, ISUTransactionType};
use crate::member::MemberId;
use async_trait::async_trait;
use shared::Result;

/// ISU账户Repository trait
#[async_trait]
pub trait ISUAccountRepository: Send + Sync {
    /// 保存ISU账户
    async fn save(&self, account: &ISUAccount) -> Result<()>;

    /// 根据ID查找账户
    async fn find_by_id(&self, id: &ISUAccountId) -> Result<Option<ISUAccount>>;

    /// 根据所有者ID查找账户
    async fn find_by_owner_id(&self, owner_id: &MemberId) -> Result<Option<ISUAccount>>;

    /// 执行ISU转账（原子操作）
    async fn transfer(
        &self,
        from_account_id: &ISUAccountId,
        to_account_id: &ISUAccountId,
        amount: &ISU,
        transaction_type: ISUTransactionType,
        description: Option<String>,
    ) -> Result<ISUTransaction>;

    /// 获取账户交易历史
    async fn get_transaction_history(
        &self,
        account_id: &ISUAccountId,
        limit: Option<u32>,
    ) -> Result<Vec<ISUTransaction>>;

    /// 更新账户余额（管理员操作）
    async fn update_balance(&self, account_id: &ISUAccountId, new_balance: &ISU) -> Result<()>;
}