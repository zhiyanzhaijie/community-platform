//! Server Functions 桥接层
#![cfg_attr(feature = "server", allow(unused_imports))]

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

// ========== 导入 core/api 的 DTO（不启用 openapi feature） ==========
#[cfg(feature = "server")]
pub use core_api::dto::member::{LoginRequest, LoginResponse, MemberDto, RegisterRequest};

// 前端也需要这些类型，但不能直接 re-export，因为前端不依赖 api-dto
// 所以我们在这里定义前端版本，与 core/api 的 DTO 保持结构一致
#[cfg(not(feature = "server"))]
mod dto {
    use super::*;

    #[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
    pub struct LoginRequest {
        pub email: String,
        pub password: String,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
    pub struct LoginResponse {
        pub token: String,
        pub member: MemberDto,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
    pub struct RegisterRequest {
        pub email: String,
        pub username: String,
        pub password: String,
    }

    #[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
    pub struct MemberDto {
        pub id: String,
        pub email: String,
        pub username: String,
        pub status: String,
        pub created_at: String,
    }
}

#[cfg(not(feature = "server"))]
pub use dto::{LoginRequest, LoginResponse, MemberDto, RegisterRequest};

// ========== Server Functions（只在服务端编译） ==========

/// Echo the user input on the server.
#[post("/api/echo")]
pub async fn echo(input: String) -> Result<String, ServerFnError> {
    Ok(input)
}

/// 登录 Server Function
#[post("/api/auth/login")]
pub async fn login(email: String, password: String) -> Result<LoginResponse, ServerFnError> {
    #[cfg(feature = "server")]
    {
        use crate::server_impl::*;

        // 从上下文中获取状态
        let state = get_server_state().await?;

        // 调用 Core 业务逻辑
        perform_login(&state, email, password).await
    }

    #[cfg(not(feature = "server"))]
    {
        // 编译到 WASM 时，这段代码会被优化掉
        unreachable!("This function only runs on the server")
    }
}

/// 注册 Server Function
#[post("/api/auth/register")]
pub async fn register(
    email: String,
    username: String,
    password: String,
) -> Result<MemberDto, ServerFnError> {
    #[cfg(feature = "server")]
    {
        use crate::server_impl::*;

        let state = get_server_state().await?;

        perform_register(&state, email, username, password).await
    }

    #[cfg(not(feature = "server"))]
    {
        unreachable!("This function only runs on the server")
    }
}

// ========== 服务端实现（只在服务端编译） ==========

#[cfg(feature = "server")]
mod server_impl {
    use super::*;
    use std::sync::Arc;

    /// 服务端状态（复用 core/api 的结构）
    #[derive(Clone)]
    pub struct ServerState {
        pub member_repo: Arc<dyn domain::member::MemberRepository>,
        pub tool_repo: Arc<dyn domain::tool::ToolRepository>,
        pub password_hasher: Arc<dyn infra::PasswordHasher>,
        pub config: Arc<shared::AppConfig>,
    }

    /// 从 Dioxus Context 中获取服务端状态
    pub async fn get_server_state() -> Result<ServerState, ServerFnError> {
        dioxus::prelude::try_consume_context::<ServerState>()
            .ok_or_else(|| ServerFnError::new("Server state not found"))
    }

    /// 执行登录逻辑（复用 core 的业务逻辑）
    pub async fn perform_login(
        state: &ServerState,
        email: String,
        password: String,
    ) -> Result<LoginResponse, ServerFnError> {
        use app::member::{login_member, LoginInput};

        // 调用 Core 业务逻辑
        let output = login_member(
            state.member_repo.as_ref(),
            state.password_hasher.as_ref(),
            LoginInput { email, password },
        )
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

        // 生成 JWT token（复用 core/api 的函数）
        let token = generate_token(
            &output.member.id.to_string(),
            &state.config.jwt.secret,
            state.config.jwt.expires_in as u64,
        )
        .map_err(|e| ServerFnError::new(e.to_string()))?;

        // 转换为 DTO（与 core/api/dto 保持一致）
        let member = MemberDto {
            id: output.member.id.to_string(),
            email: output.member.email.value().to_string(),
            username: output.member.username.value().to_string(),
            status: output.member.status.to_string(),
            created_at: output.member.created_at.to_rfc3339(),
        };

        Ok(LoginResponse { token, member })
    }

    /// 执行注册逻辑（复用 core 的业务逻辑）
    pub async fn perform_register(
        state: &ServerState,
        email: String,
        username: String,
        password: String,
    ) -> Result<MemberDto, ServerFnError> {
        use app::member::{register_member, RegisterInput};

        let member = register_member(
            state.member_repo.as_ref(),
            state.password_hasher.as_ref(),
            RegisterInput {
                email,
                username,
                password,
            },
        )
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

        // 转换为 DTO
        Ok(MemberDto {
            id: member.id.to_string(),
            email: member.email.value().to_string(),
            username: member.username.value().to_string(),
            status: member.status.to_string(),
            created_at: member.created_at.to_rfc3339(),
        })
    }

    /// 生成 JWT Token（复用 core/api 的逻辑）
    fn generate_token(
        user_id: &str,
        secret: &str,
        expires_in: u64,
    ) -> Result<String, anyhow::Error> {
        use jsonwebtoken::{encode, EncodingKey, Header};

        #[derive(Debug, serde::Serialize, serde::Deserialize)]
        struct Claims {
            sub: String,
            exp: usize,
        }

        let expiration = chrono::Utc::now()
            .checked_add_signed(chrono::Duration::seconds(expires_in as i64))
            .ok_or_else(|| anyhow::anyhow!("Invalid timestamp"))?
            .timestamp();

        let claims = Claims {
            sub: user_id.to_owned(),
            exp: expiration as usize,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .map_err(|e| anyhow::anyhow!("Failed to generate token: {}", e))
    }
}

// 导出 ServerState 供 web 入口使用
#[cfg(feature = "server")]
pub use server_impl::ServerState;
