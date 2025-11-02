//! 会员 DTO

use domain::member::Member;
use serde::{Deserialize, Serialize};
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

/// 注册请求
#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct RegisterRequest {
    pub email: String,
    pub username: String,
    pub password: String,
}

/// 登录请求
#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

/// 登录响应
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct LoginResponse {
    pub token: String,
    pub member: MemberDto,
}

/// 会员 DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct MemberDto {
    pub id: String,
    pub email: String,
    pub username: String,
    pub status: String,
    pub created_at: String,
}

/// Member -> MemberDto 转换
impl From<&Member> for MemberDto {
    fn from(member: &Member) -> Self {
        Self {
            id: member.id.to_string(),
            email: member.email.value().to_string(),
            username: member.username.value().to_string(),
            status: member.status.to_string(),
            created_at: member.created_at.to_rfc3339(),
        }
    }
}
