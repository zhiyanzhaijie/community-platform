use dioxus::prelude::*;
use crate::types::{PaginatedResponse, ToolDto};

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
