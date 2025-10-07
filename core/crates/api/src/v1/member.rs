//! 会员 API 端点

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
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
use shared::AppConfig;

/// 应用状态
#[derive(Clone)]
pub struct AppState<R: MemberRepository> {
    pub member_repo: Arc<R>,
    pub config: Arc<AppConfig>,
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

    let member = register_member(state.member_repo.as_ref(), input).await?;

    let dto = MemberDto {
        id: member.id.to_string(),
        email: member.email.value().to_string(),
        username: member.username.value().to_string(),
        status: member.status.to_string(),
        created_at: member.created_at.to_rfc3339(),
    };

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

    let output = login_member(state.member_repo.as_ref(), input).await?;

    // 生成 JWT token
    let token = generate_token(
        &output.member.id.to_string(),
        &state.config.jwt.secret,
        state.config.jwt.expires_in,
    )?;

    let dto = MemberDto {
        id: output.member.id.to_string(),
        email: output.member.email.value().to_string(),
        username: output.member.username.value().to_string(),
        status: output.member.status.to_string(),
        created_at: output.member.created_at.to_rfc3339(),
    };

    let response = LoginResponse { token, member: dto };

    Ok(Json(ApiResponse::success(response)))
}

/// 错误处理
use shared::AppError;

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Validation(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "未授权".to_string()),
            AppError::Forbidden => (StatusCode::FORBIDDEN, "权限不足".to_string()),
            AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::Config(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("配置错误: {}", e),
            ),
        };

        let body = Json(ApiResponse::<()>::error(message));
        (status, body).into_response()
    }
}
