//! 会员 API 端点

use axum::{
    extract::State,
    routing::post,
    Json, Router,
};
use std::sync::Arc;

use crate::{
    dto::{
        common::ApiResponse,
        member::{LoginRequest, LoginResponse, MemberDto, RegisterRequest},
    },
    middleware::auth::generate_token,
};
use app::member::{login_member, register_member, LoginInput, RegisterInput};
use domain::member::MemberRepository;
use shared::{AppConfig, AppError};

/// 应用状态
pub struct AppState<R: MemberRepository> {
    pub member_repo: Arc<R>,
    pub password_hasher: Arc<dyn infra::PasswordHasher>,
    pub config: Arc<AppConfig>,
}

// 手动实现 Clone（因为 R 不一定实现 Clone）
impl<R: MemberRepository> Clone for AppState<R> {
    fn clone(&self) -> Self {
        Self {
            member_repo: Arc::clone(&self.member_repo),
            password_hasher: Arc::clone(&self.password_hasher),
            config: Arc::clone(&self.config),
        }
    }
}

/// 会员路由
pub fn routes<R: MemberRepository + 'static>() -> Router<AppState<R>> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
}

/// 注册
async fn register<R: MemberRepository>(
    State(state): State<AppState<R>>,
    Json(req): Json<RegisterRequest>,
) -> Result<Json<ApiResponse<MemberDto>>, AppError> {
    let input = RegisterInput {
        email: req.email,
        username: req.username,
        password: req.password,
    };

    let member = register_member(
        state.member_repo.as_ref(),
        state.password_hasher.as_ref(),
        input,
    )
    .await?;
    let dto = MemberDto::from(&member);
    Ok(Json(ApiResponse::success(dto)))
}

/// 登录
async fn login<R: MemberRepository>(
    State(state): State<AppState<R>>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<ApiResponse<LoginResponse>>, AppError> {
    let input = LoginInput {
        email: req.email,
        password: req.password,
    };

    let output = login_member(
        state.member_repo.as_ref(),
        state.password_hasher.as_ref(),
        input,
    )
    .await?;

    // 生成 JWT token
    let token = generate_token(
        &output.member.id.to_string(),
        &state.config.jwt.secret,
        state.config.jwt.expires_in,
    )?;

    let dto = MemberDto::from(&output.member);
    let response = LoginResponse { token, member: dto };

    Ok(Json(ApiResponse::success(response)))
}

