#![cfg(feature = "server")]

use crate::types::ApiResponse;
use dioxus::prelude::*;
#[cfg(feature = "server")]
use dioxus_fullstack::FullstackContext;
use reqwest::{Client, Method, RequestBuilder};
use serde::de::DeserializeOwned;
use std::sync::OnceLock;

static CLIENT: OnceLock<Client> = OnceLock::new();

fn get_client() -> &'static Client {
    CLIENT.get_or_init(|| Client::builder().no_proxy().build().unwrap_or_default())
}

const API_BASE_URL: &str = "http://localhost:3000/api/v1";

/// 核心逻辑：构建请求 + 注入 Auth
async fn build_request(method: Method, path: &str) -> RequestBuilder {
    let url = format!("{}{}", API_BASE_URL, path);
    let mut req = get_client().request(method, &url);

    // 1. 尝试从 Cookie 提取 Token (使用 FullstackContext::extract)
    // extract::<reqwest::header::HeaderMap>
    #[cfg(feature = "server")]
    if let Ok(headers) = FullstackContext::extract::<reqwest::header::HeaderMap, _>().await {
        if let Some(cookie) = headers.get("cookie").and_then(|c| c.to_str().ok()) {
            for part in cookie.split(';') {
                let part = part.trim();
                if let Some(token) = part.strip_prefix("token=") {
                    req = req.header("Authorization", format!("Bearer {}", token));
                    break;
                }
            }
        }
    }

    // 2. 环境变量覆盖 (Dev Mode)
    if let Ok(env_token) = std::env::var("API_TOKEN") {
        req = req.header("Authorization", format!("Bearer {}", env_token));
    }

    req
}

/// 核心逻辑：执行请求 + 统一错误处理 + 解包 ApiResponse
async fn execute<T>(req: RequestBuilder) -> Result<T, ServerFnError>
where
    T: DeserializeOwned,
{
    let resp = req
        .send()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let status = resp.status();
    if !status.is_success() {
        let body = resp.text().await.unwrap_or_default();
        return Err(ServerFnError::new(format!(
            "API Error {}: {}",
            status, body
        )));
    }

    let api_res: ApiResponse<T> = resp
        .json()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    match api_res.code {
        200 => api_res
            .data
            .ok_or_else(|| ServerFnError::new("No data returned")),
        _ => Err(ServerFnError::new(api_res.message.unwrap_or_default())),
    }
}

pub async fn request<T>(method: Method, path: &str) -> Result<T, ServerFnError>
where
    T: DeserializeOwned,
{
    let req = build_request(method, path).await;
    execute(req).await
}

pub async fn request_json<T, B>(method: Method, path: &str, body: &B) -> Result<T, ServerFnError>
where
    T: DeserializeOwned,
    B: serde::Serialize,
{
    let req = build_request(method, path).await.json(body);
    execute(req).await
}
