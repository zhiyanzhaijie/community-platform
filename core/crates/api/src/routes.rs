//! 路由配置

use axum::{http::StatusCode, middleware, response::Json, routing::get, Router};
use serde_json::{json, Value};
use shared::constants::API_VERSION_V1;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::AppState;
use crate::openapi::ApiDoc;
use crate::middleware::auth::{auth_middleware_with_secret, Claims};
use axum::extract::Request;
use tower::Layer;

pub fn app_routes(state: AppState) -> Router {
    let router = Router::new()
        .merge(SwaggerUi::new("/api/docs").url("/api/openapi.json", ApiDoc::openapi()))
        .nest(API_VERSION_V1, v1_routes(state))
        .route("/health", get(health_check))
        .route("/ready", get(readiness_check));
    
    router
}

fn v1_routes(state: AppState) -> Router {
    let secret = state.config.jwt.secret.clone();
    
    Router::new()
        .nest("/members", crate::v1::member::routes())
        .nest(
            "/tools",
            crate::v1::tool::routes()
                .layer(middleware::from_fn(move |req: Request, next| {
                    auth_middleware_with_secret(req, next, secret.clone())
                })),
        )
        .with_state(state)
}

async fn health_check() -> (StatusCode, Json<Value>) {
    (StatusCode::OK, Json(json!({ "status": "ok" })))
}

async fn readiness_check() -> (StatusCode, Json<Value>) {
    (StatusCode::OK, Json(json!({ "status": "ready" })))
}
