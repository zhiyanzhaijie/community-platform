//! 分配决策者权限用例

use domain::{
    member::{MemberRepository, MemberId},
    profession::ProfessionType,
};
use shared::{AppError, Result};
use tracing::{info, instrument, warn};

/// 分配决策者权限输入
#[derive(Debug)]
pub struct AssignDeciderInput {
    pub admin_id: MemberId,
    pub target_member_id: MemberId,
    pub managed_professions: Vec<ProfessionType>,
}

/// 分配决策者权限输出
#[derive(Debug)]
pub struct AssignDeciderOutput {
    pub member_id: MemberId,
    pub assigned_professions: Vec<ProfessionType>,
    pub message: String,
}

/// 分配决策者权限用例
#[instrument(
    name = "assign_decider",
    skip(member_repo),
    fields(
        admin_id = %input.admin_id,
        target_member_id = %input.target_member_id,
        professions_count = input.managed_professions.len()
    )
)]
pub async fn execute(
    member_repo: &impl MemberRepository,
    input: AssignDeciderInput,
) -> Result<AssignDeciderOutput> {
    info!("开始分配决策者权限");

    // 1. 验证管理员存在且是管理员角色
    let admin = member_repo
        .find_by_id(input.admin_id)
        .await?
        .ok_or_else(|| AppError::not_found("管理员不存在"))?;

    if !admin.is_admin() {
        warn!(
            admin_id = %input.admin_id,
            admin_role = %admin.role,
            "非管理员用户尝试分配决策者权限"
        );
        return Err(AppError::forbidden("只有管理员可以分配决策者权限"));
    }

    if !admin.is_active() {
        return Err(AppError::validation("管理员账户未激活"));
    }

    // 2. 验证目标用户存在且状态正常
    let mut target_member = member_repo
        .find_by_id(input.target_member_id)
        .await?
        .ok_or_else(|| AppError::not_found("目标用户不存在"))?;

    if !target_member.is_active() {
        return Err(AppError::validation("目标用户账户未激活"));
    }

    // 3. 验证不能给管理员分配决策者权限
    if target_member.is_admin() {
        return Err(AppError::validation("不能给管理员分配决策者权限"));
    }

    // 4. 验证职业列表不为空
    if input.managed_professions.is_empty() {
        return Err(AppError::validation("必须指定至少一个管理的职业"));
    }

    // 5. 验证职业列表不重复
    for (i, profession) in input.managed_professions.iter().enumerate() {
        if input.managed_professions[i + 1..].contains(profession) {
            return Err(AppError::validation("职业列表中包含重复项"));
        }
    }
    
    let unique_professions = input.managed_professions.clone();

    // 6. 提升为决策者
    target_member.promote_to_decider(unique_professions.clone());

    // 7. 保存更新
    member_repo.update(&target_member).await?;

    info!(
        admin_id = %input.admin_id,
        target_member_id = %input.target_member_id,
        assigned_professions = ?unique_professions,
        "决策者权限分配成功"
    );

    Ok(AssignDeciderOutput {
        member_id: input.target_member_id,
        assigned_professions: unique_professions.clone(),
        message: format!(
            "用户已成功提升为决策者，可管理职业: {}",
            unique_professions
                .iter()
                .map(|p| p.display_name())
                .collect::<Vec<_>>()
                .join(", ")
        ),
    })
}

/// 撤销决策者权限用例
#[instrument(
    name = "revoke_decider",
    skip(member_repo),
    fields(
        admin_id = %admin_id,
        target_member_id = %target_member_id
    )
)]
pub async fn revoke_decider(
    member_repo: &impl MemberRepository,
    admin_id: MemberId,
    target_member_id: MemberId,
) -> Result<String> {
    info!("开始撤销决策者权限");

    // 1. 验证管理员权限
    let admin = member_repo
        .find_by_id(admin_id)
        .await?
        .ok_or_else(|| AppError::not_found("管理员不存在"))?;

    if !admin.is_admin() {
        return Err(AppError::forbidden("只有管理员可以撤销决策者权限"));
    }

    // 2. 验证目标用户存在
    let mut target_member = member_repo
        .find_by_id(target_member_id)
        .await?
        .ok_or_else(|| AppError::not_found("目标用户不存在"))?;

    if !target_member.is_decider() {
        return Err(AppError::validation("用户不是决策者"));
    }

    // 3. 降级为普通用户
    target_member.demote_to_regular();

    // 4. 保存更新
    member_repo.update(&target_member).await?;

    info!(
        admin_id = %admin_id,
        target_member_id = %target_member_id,
        "决策者权限撤销成功"
    );

    Ok("用户的决策者权限已撤销，降级为普通用户".to_string())
}