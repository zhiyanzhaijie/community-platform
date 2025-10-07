//! 路由配置

use axum::{
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use serde_json::{json, Value};
use shared::constants::API_VERSION_V1;

/// 应用路由
pub fn app_routes() -> Router {
    Router::new()
        .nest(API_VERSION_V1, v1_routes())
        .route("/health", get(health_check))
        .route("/ready", get(readiness_check))
}

/// v1 版本路由
fn v1_routes() -> Router {
    Router::new()
    // TODO: 添加业务路由
    // .nest("/members", crate::v1::member::routes())
    // .nest("/tools", crate::v1::tool::routes())
    // .nest("/transactions", crate::v1::transaction::routes())
}

/// 健康检查
async fn health_check() -> (StatusCode, Json<Value>) {
    (StatusCode::OK, Json(json!({ "status": "ok" })))
}

/// 就绪检查
async fn readiness_check() -> (StatusCode, Json<Value>) {
    // TODO: 检查数据库连接等
    (StatusCode::OK, Json(json!({ "status": "ready" })))
}
