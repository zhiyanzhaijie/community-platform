//! 确认交易用例

use domain::{
    isu::{ISUAccountRepository, ISUTransactionType},
    member::{MemberRepository, MemberId},
    service::ServiceRepository,
    transaction::{TransactionRepository, TransactionId},
};
use shared::{AppError, Result};
use tracing::{info, instrument};

/// 确认交易输入
#[derive(Debug)]
pub struct ConfirmTransactionInput {
    pub transaction_id: TransactionId,
    pub seller_id: MemberId, // 只有卖家可以确认交易
}

/// 确认交易输出
#[derive(Debug)]
pub struct ConfirmTransactionOutput {
    pub transaction_id: TransactionId,
    pub message: String,
}

/// 确认交易用例（卖家确认 + ISU转移）
#[instrument(
    name = "confirm_transaction",
    skip(member_repo, service_repo, isu_repo, transaction_repo),
    fields(
        transaction_id = %input.transaction_id,
        seller_id = %input.seller_id
    )
)]
pub async fn execute(
    member_repo: &impl MemberRepository,
    service_repo: &impl ServiceRepository,
    isu_repo: &impl ISUAccountRepository,
    transaction_repo: &impl TransactionRepository,
    input: ConfirmTransactionInput,
) -> Result<ConfirmTransactionOutput> {
    info!("开始确认交易");

    // 1. 验证交易存在
    let mut transaction = transaction_repo
        .find_by_id(&input.transaction_id)
        .await?
        .ok_or_else(|| AppError::not_found("交易不存在"))?;

    // 2. 验证操作者是卖家
    if !transaction.is_seller(&input.seller_id) {
        return Err(AppError::forbidden("只有卖家可以确认交易"));
    }

    // 3. 验证卖家存在且状态正常
    let seller = member_repo
        .find_by_id(input.seller_id)
        .await?
        .ok_or_else(|| AppError::not_found("卖家不存在"))?;

    if !seller.is_active() {
        return Err(AppError::validation("卖家账户未激活"));
    }

    // 4. 确认交易状态
    transaction.confirm()?;

    // 5. 执行ISU转移（买家 → 卖家）
    let buyer_isu_account = isu_repo
        .find_by_owner_id(&transaction.buyer_id)
        .await?
        .ok_or_else(|| AppError::not_found("买家ISU账户不存在"))?;

    let seller_isu_account = isu_repo
        .find_by_owner_id(&transaction.seller_id)
        .await?
        .ok_or_else(|| AppError::not_found("卖家ISU账户不存在"))?;

    // 检查买家余额（再次确认）
    if !buyer_isu_account.has_sufficient_balance(&transaction.isu_amount) {
        return Err(AppError::validation("买家ISU余额不足"));
    }

    // 执行ISU转移
    let isu_transaction = isu_repo
        .transfer(
            &buyer_isu_account.id,
            &seller_isu_account.id,
            &transaction.isu_amount,
            ISUTransactionType::ServicePayment,
            Some(format!("服务交易确认 - {}", transaction.id.value())),
        )
        .await?;

    info!(
        isu_transaction_id = isu_transaction.id,
        isu_amount = %transaction.isu_amount,
        "ISU转移成功"
    );

    // 6. 自动开始交易
    transaction.start()?;

    // 7. 更新相关服务状态为进行中
    if let domain::transaction::TransactionItemType::Service(service_id) = &transaction.item_type {
        if let Some(mut service) = service_repo.find_by_id(service_id).await? {
            service.start()?;
            service_repo.save(&service).await?;
        }
    }

    // 8. 保存交易状态更新
    transaction_repo.update(&transaction).await?;

    info!(
        transaction_id = %input.transaction_id,
        new_status = %transaction.status,
        "交易确认完成"
    );

    Ok(ConfirmTransactionOutput {
        transaction_id: input.transaction_id,
        message: "交易已确认并开始进行，ISU已转移".to_string(),
    })
}