//! 完成交易用例

use domain::{
    member::{MemberRepository, MemberId},
    service::ServiceRepository,
    transaction::{TransactionRepository, TransactionId},
};
use shared::{AppError, Result};
use tracing::{info, instrument};

/// 完成交易输入
#[derive(Debug)]
pub struct CompleteTransactionInput {
    pub transaction_id: TransactionId,
    pub requester_id: MemberId, // 买家或卖家都可以标记完成
}

/// 完成交易输出
#[derive(Debug)]
pub struct CompleteTransactionOutput {
    pub transaction_id: TransactionId,
    pub message: String,
}

/// 完成交易用例
#[instrument(
    name = "complete_transaction",
    skip(member_repo, service_repo, transaction_repo),
    fields(
        transaction_id = %input.transaction_id,
        requester_id = %input.requester_id
    )
)]
pub async fn execute(
    member_repo: &impl MemberRepository,
    service_repo: &impl ServiceRepository,
    transaction_repo: &impl TransactionRepository,
    input: CompleteTransactionInput,
) -> Result<CompleteTransactionOutput> {
    info!("开始完成交易");

    // 1. 验证交易存在
    let mut transaction = transaction_repo
        .find_by_id(&input.transaction_id)
        .await?
        .ok_or_else(|| AppError::not_found("交易不存在"))?;

    // 2. 验证操作者是交易参与者
    if !transaction.is_participant(&input.requester_id) {
        return Err(AppError::forbidden("只有交易参与者可以完成交易"));
    }

    // 3. 验证操作者存在且状态正常
    let requester = member_repo
        .find_by_id(input.requester_id)
        .await?
        .ok_or_else(|| AppError::not_found("操作者不存在"))?;

    if !requester.is_active() {
        return Err(AppError::validation("操作者账户未激活"));
    }

    // 4. 完成交易
    transaction.complete()?;

    // 5. 更新相关服务状态为已完成
    if let domain::transaction::TransactionItemType::Service(service_id) = &transaction.item_type {
        if let Some(mut service) = service_repo.find_by_id(service_id).await? {
            service.complete()?;
            service_repo.save(&service).await?;
            info!(service_id = %service_id, "服务状态已更新为完成");
        }
    }

    // 6. 保存交易状态更新
    transaction_repo.update(&transaction).await?;

    info!(
        transaction_id = %input.transaction_id,
        completed_by = %input.requester_id,
        completed_at = ?transaction.completed_at,
        "交易完成"
    );

    let message = if transaction.is_buyer(&input.requester_id) {
        "交易已完成，感谢您的购买！".to_string()
    } else {
        "交易已完成，感谢您提供的服务！".to_string()
    };

    Ok(CompleteTransactionOutput {
        transaction_id: input.transaction_id,
        message,
    })
}