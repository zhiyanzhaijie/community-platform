//! OpenAPI 文档定义

#[cfg(feature = "openapi")]
use utoipa::OpenApi;

#[cfg(feature = "openapi")]
use crate::dto::{
    common::PaginationQuery,
    member::{LoginRequest, LoginResponse, MemberDto, RegisterRequest},
};

/// OpenAPI 文档结构
#[cfg(feature = "openapi")]
#[derive(OpenApi)]
#[cfg_attr(feature = "openapi", openapi(
    info(
        title = "Community Trading Platform API",
        version = "1.0.0",
        description = "社区交易平台 RESTful API 文档"
    ),
    paths(
        crate::v1::member::register,
        crate::v1::member::login,
    ),
    components(
        schemas(
            PaginationQuery,
            RegisterRequest,
            LoginRequest,
            LoginResponse,
            MemberDto,
        )
    ),
    tags(
        (name = "members", description = "会员管理"),
    )
))]
pub struct ApiDoc;
