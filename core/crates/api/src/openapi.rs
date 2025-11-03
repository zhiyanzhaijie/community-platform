//! OpenAPI 文档定义
//!
//! 注: 这个模块只在 http-server feature 下编译

use utoipa::OpenApi;

use crate::dto::{
    common::PaginationQuery,
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
)]
pub struct ApiDoc;
