//! 会员注册用例

use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use domain::member::{Email, Member, MemberRepository, Username};
use shared::{AppError, Result};
use tracing::instrument;

/// 注册输入
pub struct RegisterInput {
    pub email: String,
    pub username: String,
    pub password: String,
}

/// 注册会员
#[instrument(
    name = "register_member",
    skip(repo, input),
    fields(
        email = %input.email,
        username = %input.username
    )
)]
pub async fn register_member<R: MemberRepository>(
    repo: &R,
    input: RegisterInput,
) -> Result<Member> {
    tracing::info!("开始注册会员");

    // 验证密码强度
    if input.password.len() < 8 {
        return Err(AppError::validation("密码长度至少8位"));
    }

    // 构建值对象
    let email = Email::new(input.email)?;
    let username = Username::new(input.username)?;

    // 检查邮箱是否已存在
    if repo.find_by_email(&email).await?.is_some() {
        return Err(AppError::validation("邮箱已被注册"));
    }

    // 检查用户名是否已存在
    if repo.find_by_username(&username).await?.is_some() {
        return Err(AppError::validation("用户名已被使用"));
    }

    // 哈希密码
    let password_hash = hash_password(&input.password)?;

    // 创建会员
    let member = Member::new(email, username, password_hash);

    // 保存到仓储
    repo.save(&member).await?;

    tracing::info!(member_id = %member.id, "会员注册成功");
    Ok(member)
}

/// 哈希密码
fn hash_password(password: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|e| AppError::internal(format!("密码哈希失败: {}", e)))
}
