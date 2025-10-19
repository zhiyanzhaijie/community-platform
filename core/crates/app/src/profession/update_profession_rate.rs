//! 更新职业标准费率用例

use domain::{
    isu::ISURate,
    member::{MemberRepository, MemberId},
    profession::{ProfessionStandardEntity, ProfessionStandardHistory, ProfessionStandardRepository, ProfessionType, StandardAction},
};
use rust_decimal::Decimal;
use shared::{AppError, Result};
use tracing::{info, instrument, warn};
use uuid::Uuid;

/// 更新职业标准费率输入
#[derive(Debug)]
pub struct UpdateProfessionRateInput {
    pub requester_id: MemberId,
    pub profession_type: ProfessionType,
    pub new_rate: Decimal,
    pub reason: String,
}

/// 更新职业标准费率输出
#[derive(Debug)]
pub struct UpdateProfessionRateOutput {
    pub profession_type: ProfessionType,
    pub old_rate: ISURate,
    pub new_rate: ISURate,
    pub message: String,
}

/// 更新职业标准费率用例
#[instrument(
    name = "update_profession_rate",
    skip(member_repo, profession_repo),
    fields(
        requester_id = %input.requester_id,
        profession_type = %input.profession_type,
        new_rate = %input.new_rate
    )
)]
pub async fn execute(
    member_repo: &impl MemberRepository,
    profession_repo: &impl ProfessionStandardRepository,
    input: UpdateProfessionRateInput,
) -> Result<UpdateProfessionRateOutput> {
    info!("开始更新职业标准费率");

    // 1. 验证请求者存在且状态正常
    let requester = member_repo
        .find_by_id(input.requester_id)
        .await?
        .ok_or_else(|| AppError::not_found("请求者不存在"))?;

    if !requester.is_active() {
        return Err(AppError::validation("请求者账户未激活"));
    }

    // 2. 验证权限：必须是管理员或该职业的决策者
    if !requester.can_manage_profession(&input.profession_type) {
        warn!(
            requester_id = %input.requester_id,
            profession_type = %input.profession_type,
            role = %requester.role,
            "用户尝试更新无权限管理的职业标准"
        );
        return Err(AppError::forbidden("您没有权限管理此职业标准"));
    }

    // 3. 验证新费率有效性
    if input.new_rate <= Decimal::ZERO {
        return Err(AppError::validation("费率必须大于0"));
    }

    let new_isu_rate = ISURate::from_f64(input.new_rate.try_into().map_err(|_| {
        AppError::validation("无效的费率数值")
    })?)?;

    // 4. 查找当前活跃的职业标准
    let current_standard = profession_repo
        .find_active_by_profession(input.profession_type)
        .await?;

    let (standard_entity, old_rate) = if let Some(mut standard) = current_standard {
        // 更新现有标准
        let old_rate = standard.isu_rate;
        
        // 检查是否有实际变化
        if old_rate.value() == new_isu_rate.value() {
            return Err(AppError::validation("新费率与当前费率相同"));
        }

        standard.update_rate(
            new_isu_rate,
            Some(format!("费率调整：{}", input.reason)),
            input.requester_id,
        )?;

        profession_repo.update(&standard).await?;
        
        (standard, old_rate)
    } else {
        // 创建新的标准（如果不存在）
        let default_rate = input.profession_type.default_rate()?;
        let standard = ProfessionStandardEntity::new(
            input.profession_type,
            new_isu_rate,
            format!("初始设定：{}", input.reason),
            input.requester_id,
        );

        profession_repo.save(&standard).await?;
        
        (standard, default_rate)
    };

    // 5. 记录变更历史
    let history = ProfessionStandardHistory {
        id: Uuid::new_v4().to_string(),
        standard_id: standard_entity.id,
        action: StandardAction::RateUpdated,
        old_rate: Some(old_rate),
        new_rate: Some(new_isu_rate),
        reason: input.reason.clone(),
        changed_by: input.requester_id,
        created_at: chrono::Utc::now(),
    };

    profession_repo.save_history(&history).await?;

    info!(
        profession_type = %input.profession_type,
        old_rate = %old_rate,
        new_rate = %new_isu_rate,
        updated_by = %input.requester_id,
        "职业标准费率更新成功"
    );

    Ok(UpdateProfessionRateOutput {
        profession_type: input.profession_type,
        old_rate,
        new_rate: new_isu_rate,
        message: format!(
            "{}的标准费率已从 {} 更新为 {}",
            input.profession_type.display_name(),
            old_rate,
            new_isu_rate
        ),
    })
}