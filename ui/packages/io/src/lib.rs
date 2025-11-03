//! Server Functions 桥接层
//!
//! 这个模块作为 Dioxus fullstack 和 Core 业务逻辑之间的桥接:
//! - 前端: 使用 server functions 调用后端
//! - 后端: 调用 Core 的业务逻辑，返回 DTO

use dioxus::prelude::*;

// ========== 导入 core/api 的 DTO（前后端都可用） ==========
pub use api_dto::dto::member::{LoginRequest, LoginResponse, MemberDto, RegisterRequest};

// ========== Server Functions（只在服务端编译） ==========

/// Echo the user input on the server.
#[post("/api/echo")]
pub async fn echo(input: String) -> Result<String, ServerFnError> {
    Ok(input)
}

/// 登录 Server Function
#[post("/api/auth/login")]
pub async fn login(req: LoginRequest) -> Result<LoginResponse, ServerFnError> {
    #[cfg(feature = "server")]
    {
        use crate::server_impl::{auth, get_server_state};
        let state = get_server_state().await?;
        auth::login(&state, req).await
    }

    #[cfg(not(feature = "server"))]
    {
        unreachable!("This function only runs on the server")
    }
}

/// 注册 Server Function
#[post("/api/auth/register")]
pub async fn register(req: RegisterRequest) -> Result<MemberDto, ServerFnError> {
    #[cfg(feature = "server")]
    {
        use crate::server_impl::{auth, get_server_state};
        let state = get_server_state().await?;
        auth::register(&state, req).await
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

    /// 服务端状态
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

    /// 认证相关的 server functions 实现
    pub mod auth {
        use super::*;
        use api_dto::dto::member::{LoginRequest, LoginResponse, MemberDto, RegisterRequest};

        /// 执行登录逻辑
        pub async fn login(
            state: &ServerState,
            req: LoginRequest,
        ) -> Result<LoginResponse, ServerFnError> {
            use app::member::{login_member, LoginInput};

            // 调用 Core 业务逻辑
            let output = login_member(
                state.member_repo.as_ref(),
                state.password_hasher.as_ref(),
                LoginInput {
                    email: req.email,
                    password: req.password,
                },
            )
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

            // 生成 JWT token
            let token = generate_token(
                &output.member.id.to_string(),
                &state.config.jwt.secret,
                state.config.jwt.expires_in as u64,
            )
            .map_err(|e| ServerFnError::new(e.to_string()))?;

            // 使用 api_dto 的转换
            Ok(LoginResponse {
                token,
                member: MemberDto::from(&output.member),
            })
        }

        /// 执行注册逻辑
        pub async fn register(
            state: &ServerState,
            req: RegisterRequest,
        ) -> Result<MemberDto, ServerFnError> {
            use app::member::{register_member, RegisterInput};

            let member = register_member(
                state.member_repo.as_ref(),
                state.password_hasher.as_ref(),
                RegisterInput {
                    email: req.email,
                    username: req.username,
                    password: req.password,
                },
            )
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;

            // 使用 api_dto 的转换
            Ok(MemberDto::from(&member))
        }

        /// 生成 JWT Token
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
}

// 导出 ServerState 供 web 入口使用
#[cfg(feature = "server")]
pub use server_impl::ServerState;
