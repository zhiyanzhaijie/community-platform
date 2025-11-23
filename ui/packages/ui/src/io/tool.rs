use dioxus::prelude::*;
use crate::types::{PaginatedResponse, ToolDto, CreateToolRequest, UpdateToolRequest};

#[server]
pub async fn list_tools(page: i64, page_size: i64) -> Result<PaginatedResponse<ToolDto>, ServerFnError> {
    #[cfg(feature = "server")]
    {
        use crate::io::base;
        use reqwest::Method;
        
        let path = format!("/tools?page={}&page_size={}", page, page_size);
        base::request(Method::GET, &path).await
    }
    #[cfg(not(feature = "server"))]
    unreachable!("This code is replaced by the #[server] macro on the client")
}

#[server]
pub async fn get_tool(id: String) -> Result<ToolDto, ServerFnError> {
    #[cfg(feature = "server")]
    {
        use crate::io::base;
        use reqwest::Method;
        
        let path = format!("/tools/{}", id);
        base::request(Method::GET, &path).await
    }
    #[cfg(not(feature = "server"))]
    unreachable!("This code is replaced by the #[server] macro on the client")
}

#[server]
pub async fn create_tool(req: CreateToolRequest) -> Result<ToolDto, ServerFnError> {
    #[cfg(feature = "server")]
    {
        use crate::io::base;
        use reqwest::Method;
        
        base::request_json(Method::POST, "/tools", &req).await
    }
    #[cfg(not(feature = "server"))]
    unreachable!("This code is replaced by the #[server] macro on the client")
}

#[server]
pub async fn update_tool(id: String, req: UpdateToolRequest) -> Result<ToolDto, ServerFnError> {
    #[cfg(feature = "server")]
    {
        use crate::io::base;
        use reqwest::Method;
        
        let path = format!("/tools/{}", id);
        base::request_json(Method::PUT, &path, &req).await
    }
    #[cfg(not(feature = "server"))]
    unreachable!("This code is replaced by the #[server] macro on the client")
}

#[server]
pub async fn delete_tool(id: String) -> Result<(), ServerFnError> {
    #[cfg(feature = "server")]
    {
        use crate::io::base;
        use reqwest::Method;
        
        let path = format!("/tools/{}", id);
        base::request(Method::DELETE, &path).await
    }
    #[cfg(not(feature = "server"))]
    unreachable!("This code is replaced by the #[server] macro on the client")
}

#[server]
pub async fn list_tools_by_owner(owner_id: String, page: i64, page_size: i64) -> Result<PaginatedResponse<ToolDto>, ServerFnError> {
    #[cfg(feature = "server")]
    {
        use crate::io::base;
        use reqwest::Method;
        
        let path = format!("/tools/owner/{}?page={}&page_size={}", owner_id, page, page_size);
        base::request(Method::GET, &path).await
    }
    #[cfg(not(feature = "server"))]
    unreachable!("This code is replaced by the #[server] macro on the client")
}
