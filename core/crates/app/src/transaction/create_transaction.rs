//! 创建交易用例

use domain::{
    isu::{ISU, ISUAccountRepository},
    member::{MemberRepository, MemberId},
    service::{ServiceRepository, ServiceId},
    transaction::{Transaction, TransactionItemType, TransactionRepository},
};
use shared::{AppError, Result};
use tracing::{info, instrument, warn};

/// 创建交易输入
#[derive(Debug)]
pub struct CreateTransactionInput {
    pub buyer_id: MemberId,
    pub service_id: ServiceId,
    pub description: Option<String>,
}

/// 创建交易输出
#[derive(Debug)]
pub struct CreateTransactionOutput {
    pub transaction_id: domain::transaction::TransactionId,
    pub isu_amount: ISU,
    pub seller_id: MemberId,
}

/// 创建交易用例
#[instrument(
    name = "create_transaction",
    skip(member_repo, service_repo, isu_repo, transaction_repo),
    fields(
        buyer_id = %input.buyer_id,
        service_id = %input.service_id
    )
)]
pub async fn execute(
    member_repo: &impl MemberRepository,
    service_repo: &impl ServiceRepository,
    isu_repo: &impl ISUAccountRepository,
    transaction_repo: &impl TransactionRepository,
    input: CreateTransactionInput,
) -> Result<CreateTransactionOutput> {
    info!("开始创建交易");

    // 1. 验证买家存在且状态正常
    let buyer = member_repo
        .find_by_id(input.buyer_id)
        .await?
        .ok_or_else(|| AppError::not_found("买家不存在"))?;

    if !buyer.is_active() {
        return Err(AppError::validation("买家账户未激活"));
    }

    // 2. 验证服务存在且可用
    let service = service_repo
        .find_by_id(&input.service_id)
        .await?
        .ok_or_else(|| AppError::not_found("服务不存在"))?;

    if !service.is_available() {
        return Err(AppError::validation("服务当前不可用"));
    }

    // 3. 验证买家不是服务提供者（不能自己买自己的服务）
    if service.is_owned_by(&input.buyer_id) {
        return Err(AppError::validation("不能购买自己发布的服务"));
    }

    // 4. 验证卖家存在且状态正常
    let seller = member_repo
        .find_by_id(service.provider_id)
        .await?
        .ok_or_else(|| AppError::not_found("服务提供者不存在"))?;

    if !seller.is_active() {
        return Err(AppError::validation("服务提供者账户未激活"));
    }

    // 5. 检查买家ISU余额
    let buyer_isu_account = isu_repo
        .find_by_owner_id(&input.buyer_id)
        .await?
        .ok_or_else(|| AppError::not_found("买家ISU账户不存在"))?;

    if !buyer_isu_account.has_sufficient_balance(&service.total_isu) {
        warn!(
            buyer_balance = %buyer_isu_account.get_balance(),
            required_isu = %service.total_isu,
            "买家ISU余额不足"
        );
        return Err(AppError::validation("ISU余额不足"));
    }

    // 6. 创建交易
    let transaction = Transaction::new(
        input.buyer_id,
        service.provider_id,
        TransactionItemType::Service(input.service_id),
        service.total_isu,
        input.description,
    )?;

    let transaction_id = transaction.id;
    let isu_amount = transaction.isu_amount;
    let seller_id = transaction.seller_id;

    // 7. 保存交易
    transaction_repo.save(&transaction).await?;

    info!(
        transaction_id = %transaction_id,
        isu_amount = %isu_amount,
        seller_id = %seller_id,
        "交易创建成功"
    );

    Ok(CreateTransactionOutput {
        transaction_id,
        isu_amount,
        seller_id,
    })
}