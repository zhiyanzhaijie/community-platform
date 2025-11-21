//! OpenAPI documentation

use utoipa::{OpenApi, Modify};
use utoipa::openapi::security::{SecurityScheme, HttpBuilder, HttpAuthScheme};

use crate::dto::{
    common::{ApiResponse, PaginatedResponse, PaginationQuery},
    member::{LoginRequest, LoginResponse, MemberDto, RegisterRequest},
    tool::{CreateToolRequest, ToolDto, UpdateToolRequest},
};

/// OpenAPI 文档结构
#[derive(OpenApi)]
#[openapi(
    info(
        title = "Community Trading Platform API",
        version = "1.0.0",
        description = "社区交易平台 RESTful API 文档"
    ),
    paths(
        crate::v1::member::register,
        crate::v1::member::login,
        crate::v1::tool::create_tool_handler,
        crate::v1::tool::get_tool_handler,
        crate::v1::tool::list_tools_handler,
        crate::v1::tool::update_tool_handler,
        crate::v1::tool::delete_tool_handler,
        crate::v1::tool::list_tools_by_owner_handler,
    ),
    components(
        schemas(
            ApiResponse<MemberDto>,
            ApiResponse<LoginResponse>,
            ApiResponse<ToolDto>,
            ApiResponse<PaginatedResponse<ToolDto>>,
            PaginationQuery,
            RegisterRequest,
            LoginRequest,
            LoginResponse,
            MemberDto,
            CreateToolRequest,
            UpdateToolRequest,
            ToolDto,
            PaginatedResponse<ToolDto>,
        )
    ),
    tags(
        (name = "members", description = "会员管理"),
        (name = "tools", description = "工具管理"),
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

/// Security addon for Bearer JWT token
struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(ref mut components) = openapi.components {
            components.add_security_scheme(
                "bearer_auth",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            );
        }
    }
}
