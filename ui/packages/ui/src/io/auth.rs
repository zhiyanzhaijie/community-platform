use dioxus::prelude::*;
use crate::types::{LoginRequest, LoginResponse, RegisterRequest, MemberDto};

#[server]
pub async fn login(req: LoginRequest) -> Result<LoginResponse, ServerFnError> {
    #[cfg(feature = "server")]
    {
        use crate::io::base;
        use reqwest::Method;
        base::request_json(Method::POST, "/members/login", &req).await
    }
    #[cfg(not(feature = "server"))]
    unreachable!()
}

#[server]
pub async fn register(req: RegisterRequest) -> Result<MemberDto, ServerFnError> {
    #[cfg(feature = "server")]
    {
        use crate::io::base;
        use reqwest::Method;
        base::request_json(Method::POST, "/members/register", &req).await
    }
    #[cfg(not(feature = "server"))]
    unreachable!()
}

#[server]
pub async fn get_current_member() -> Result<Option<MemberDto>, ServerFnError> {
    #[cfg(feature = "server")]
    {
        use dioxus_fullstack::FullstackContext;
        
        if let Ok(headers) = FullstackContext::extract::<reqwest::header::HeaderMap, _>().await {
            if let Some(cookie) = headers.get("cookie").and_then(|c| c.to_str().ok()) {
                for part in cookie.split(';') {
                    let part = part.trim();
                    if let Some(encoded) = part.strip_prefix("user_info=") {
                        // Decode URL-encoded JSON
                        let decoded = urlencoding::decode(encoded).unwrap_or_default();
                        if let Ok(member) = serde_json::from_str::<MemberDto>(&decoded) {
                            return Ok(Some(member));
                        }
                    }
                }
            }
        }
        Ok(None)
    }
    #[cfg(not(feature = "server"))]
    unreachable!()
}
