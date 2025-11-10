//! JWT 认证中间件

use axum::{
    async_trait,
    extract::{FromRequestParts, Request, Extension},
    http::{header::AUTHORIZATION, request::Parts, StatusCode},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use shared::AppError;
use domain::member::MemberId;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

/// 当前认证用户的ID
#[derive(Debug, Clone)]
pub struct CurrentUser(pub MemberId);

pub fn generate_token(user_id: &str, secret: &str, expires_in: i64) -> Result<String, AppError> {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::seconds(expires_in))
        .ok_or_else(|| AppError::internal("token 过期时间计算失败"))?
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| AppError::internal(format!("token 生成失败: {}", e)))
}

pub fn verify_token(token: &str, secret: &str) -> Result<Claims, AppError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|_| AppError::Unauthorized)
}

/// CurrentUser extractor - 从Extensions中提取已验证的Claims
#[async_trait]
impl<S> FromRequestParts<S> for CurrentUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // 从中间件中提取已验证的Claims
        let claims = parts
            .extensions
            .get::<Claims>()
            .cloned()
            .ok_or(AppError::Unauthorized)?;
        
        // 将user_id字符串转换为MemberId
        let user_id = shared::Id::from_string(&claims.sub)
            .map_err(|_| AppError::Unauthorized)?;

        Ok(CurrentUser(user_id))
    }
}

/// Auth middleware - 验证JWT token並将Claims组件支Extensions
pub async fn auth_middleware_with_secret(
    mut req: Request,
    next: Next,
    secret: String,
) -> Result<Response, StatusCode> {
    // 提取Authorization header
    let auth_header = req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string());

    if let Some(auth_header) = auth_header {
        // 期望格式: "Bearer <token>"
        if let Some(token) = auth_header.strip_prefix("Bearer ") {
            // 验证token
            if let Ok(claims) = verify_token(token, &secret) {
                // 将Claims存入Extensions
                req.extensions_mut().insert(claims);
                return Ok(next.run(req).await);
            }
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}

/// Legacy auth middleware for backward compatibility
pub async fn auth_middleware(req: Request, next: Next) -> Result<Response, StatusCode> {
    auth_middleware_with_secret(req, next, "your-secret-key-change-in-production".to_string()).await
}
