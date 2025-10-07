//! 会员登录用例

use argon2::{Argon2, PasswordHash, PasswordVerifier};
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
    skip(repo, input),
    fields(email = %input.email)
)]
pub async fn login_member<R: MemberRepository>(
    repo: &R,
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
    verify_password(&input.password, &member.password_hash)?;

    tracing::info!(member_id = %member.id, "会员登录成功");
    Ok(LoginOutput { member })
}

/// 验证密码
fn verify_password(password: &str, password_hash: &str) -> Result<()> {
    let parsed_hash = PasswordHash::new(password_hash)
        .map_err(|e| AppError::internal(format!("密码哈希解析失败: {}", e)))?;

    let argon2 = Argon2::default();

    argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|_| AppError::validation("邮箱或密码错误"))
}
