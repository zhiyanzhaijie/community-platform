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
