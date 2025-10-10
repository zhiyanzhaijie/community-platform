//! 会员登录用例

use domain::member::{Email, Member, MemberRepository};
use shared::{AppError, Result};
use tracing::instrument;

/// 登录输入
pub struct LoginInput {
    pub email: String,
    pub password: String,
}

/// 登录输出
pub struct LoginOutput {
    pub member: Member,
}

/// 会员登录
#[instrument(
    name = "login_member",
    skip(repo, hasher, input),
    fields(email = %input.email)
)]
pub async fn login_member(
    repo: &dyn MemberRepository,
    hasher: &dyn infra::PasswordHasher,
    input: LoginInput,
) -> Result<LoginOutput> {
    tracing::info!("开始会员登录");

    // 构建邮箱值对象
    let email = Email::new(input.email)?;

    // 查找会员
    let member = repo
        .find_by_email(&email)
        .await?
        .ok_or_else(|| AppError::validation("邮箱或密码错误"))?;

    // 检查会员状态
    if !member.is_active() {
        return Err(AppError::validation("账号未激活或已被封禁"));
    }

    // 验证密码
    if !hasher.verify(&input.password, &member.password_hash)? {
        return Err(AppError::validation("邮箱或密码错误"));
    }

    tracing::info!(member_id = %member.id, "会员登录成功");
    Ok(LoginOutput { member })
}

