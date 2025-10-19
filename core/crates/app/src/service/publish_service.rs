//! 发布服务用例

use domain::{
    member::{MemberRepository, MemberId},
    profession::ProfessionType,
    service::{Service, ServiceRepository},
};
use rust_decimal::Decimal;
use shared::{AppError, Result};
use tracing::{info, instrument};

/// 发布服务输入
#[derive(Debug)]
pub struct PublishServiceInput {
    pub provider_id: MemberId,
    pub profession_type: ProfessionType,
    pub title: String,
    pub description: String,
    pub estimated_hours: Decimal,
}

/// 发布服务输出
#[derive(Debug)]
pub struct PublishServiceOutput {
    pub service_id: domain::service::ServiceId,
    pub total_isu: domain::isu::ISU,
}

/// 发布服务用例
#[instrument(
    name = "publish_service",
    skip(member_repo, service_repo),
    fields(
        provider_id = %input.provider_id,
        profession_type = %input.profession_type,
        estimated_hours = %input.estimated_hours
    )
)]
pub async fn execute(
    member_repo: &impl MemberRepository,
    service_repo: &impl ServiceRepository,
    input: PublishServiceInput,
) -> Result<PublishServiceOutput> {
    info!("开始发布服务");

    // 1. 验证提供者存在且状态正常
    let provider = member_repo
        .find_by_id(input.provider_id)
        .await?
        .ok_or_else(|| AppError::not_found("提供者不存在"))?;

    if !provider.is_active() {
        return Err(AppError::validation("提供者账户未激活"));
    }

    // 2. 创建服务实体
    let service = Service::new(
        input.provider_id,
        input.profession_type,
        input.title,
        input.description,
        input.estimated_hours,
    )?;

    let service_id = service.id;
    let total_isu = service.total_isu;

    // 3. 保存服务
    service_repo.save(&service).await?;

    info!(
        service_id = %service_id,
        total_isu = %total_isu,
        "服务发布成功"
    );

    Ok(PublishServiceOutput {
        service_id,
        total_isu,
    })
}