//! Transaction Repository接口

use super::{Transaction, TransactionId};
use crate::member::MemberId;
use async_trait::async_trait;
use shared::Result;

/// 交易Repository trait
#[async_trait]
pub trait TransactionRepository: Send + Sync {
    /// 保存交易
    async fn save(&self, transaction: &Transaction) -> Result<()>;

    /// 根据ID查找交易
    async fn find_by_id(&self, id: &TransactionId) -> Result<Option<Transaction>>;

    /// 根据买家ID查找交易
    async fn find_by_buyer_id(&self, buyer_id: &MemberId) -> Result<Vec<Transaction>>;

    /// 根据卖家ID查找交易
    async fn find_by_seller_id(&self, seller_id: &MemberId) -> Result<Vec<Transaction>>;

    /// 根据参与者ID查找交易（买家或卖家）
    async fn find_by_participant(&self, member_id: &MemberId) -> Result<Vec<Transaction>>;

    /// 查找待确认的交易（卖家需要确认的）
    async fn find_pending_by_seller(&self, seller_id: &MemberId) -> Result<Vec<Transaction>>;

    /// 查找进行中的交易
    async fn find_in_progress_by_participant(&self, member_id: &MemberId) -> Result<Vec<Transaction>>;

    /// 更新交易状态
    async fn update(&self, transaction: &Transaction) -> Result<()>;

    /// 删除交易
    async fn delete(&self, id: &TransactionId) -> Result<()>;

    /// 统计用户交易数量
    async fn count_by_participant(&self, member_id: &MemberId) -> Result<u64>;

    /// 统计已完成交易数量
    async fn count_completed_by_participant(&self, member_id: &MemberId) -> Result<u64>;
}