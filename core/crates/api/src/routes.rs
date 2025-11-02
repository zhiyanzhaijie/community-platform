//! 路由配置

use axum::{http::StatusCode, response::Json, routing::get, Router};
use serde_json::{json, Value};
use shared::constants::API_VERSION_V1;
#[cfg(feature = "openapi")]
use utoipa::OpenApi;
#[cfg(feature = "openapi")]
use utoipa_swagger_ui::SwaggerUi;

use crate::AppState;
#[cfg(feature = "openapi")]
use crate::openapi::ApiDoc;

/// 应用路由
pub fn app_routes(state: AppState) -> Router {
    let router = Router::new();
    
    #[cfg(feature = "openapi")]
    {
        router = router.merge(SwaggerUi::new("/api/docs").url("/api/openapi.json", ApiDoc::openapi()));
    }
    
    router
        .nest(API_VERSION_V1, v1_routes(state))
        .route("/health", get(health_check))
        .route("/ready", get(readiness_check))
}

/// v1 版本路由
fn v1_routes(state: AppState) -> Router {
    Router::new()
        .nest("/members", crate::v1::member::routes())
        .nest("/tools", crate::v1::tool::routes())
        .with_state(state)
    // TODO: 添加其他业务路由
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
