//! Member DTOs

use domain::member::Member;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub struct RegisterRequest {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct LoginResponse {
    pub token: String,
    pub member: MemberDto,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MemberDto {
    pub id: String,
    pub email: String,
    pub username: String,
    pub status: String,
    pub created_at: String,
}

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
