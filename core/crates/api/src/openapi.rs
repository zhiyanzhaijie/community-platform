//! OpenAPI documentation

use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::{Modify, OpenApi};

use crate::dto::{
    common::{ApiResponse, PaginatedResponse, PaginationQuery},
    knowledge::{CreateKnowledgeRequest, KnowledgeDto, UpdateKnowledgeRequest},
    member::{LoginRequest, LoginResponse, MemberDto, RegisterRequest},
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
        crate::v1::knowledge::create_knowledge_handler,
        crate::v1::knowledge::get_knowledge_handler,
        crate::v1::knowledge::list_knowledge_handler,
        crate::v1::knowledge::update_knowledge_handler,
        crate::v1::knowledge::delete_knowledge_handler,
        crate::v1::knowledge::list_knowledge_by_owner_handler,
    ),
    components(
        schemas(
            ApiResponse<MemberDto>,
            ApiResponse<LoginResponse>,
            ApiResponse<KnowledgeDto>,
            ApiResponse<PaginatedResponse<KnowledgeDto>>,
            PaginationQuery,
            RegisterRequest,
            LoginRequest,
            LoginResponse,
            MemberDto,
            CreateKnowledgeRequest,
            UpdateKnowledgeRequest,
            KnowledgeDto,
            PaginatedResponse<KnowledgeDto>,
        )
    ),
    tags(
        (name = "members", description = "会员管理"),
        (name = "knowledge", description = "知识管理"),
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
