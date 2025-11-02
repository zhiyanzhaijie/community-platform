//! 会员 API 端点

use axum::{extract::State, routing::post, Json, Router};

use crate::{
    dto::{
        common::ApiResponse,
        member::{LoginRequest, LoginResponse, MemberDto, RegisterRequest},
    },
    middleware::auth::generate_token,
    AppState,
};
use app::member::{login_member, register_member, LoginInput, RegisterInput};
use shared::AppError;

/// 会员路由
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
}

/// 注册
#[cfg_attr(feature = "openapi", utoipa::path(post, path = "/api/v1/members/register", tag = "members"))]
async fn register(
    State(state): State<AppState>,
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
#[cfg_attr(feature = "openapi", utoipa::path(post, path = "/api/v1/members/login", tag = "members"))]
async fn login(
    State(state): State<AppState>,
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
