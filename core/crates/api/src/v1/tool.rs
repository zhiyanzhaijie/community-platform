//! 工具 API 端点

use axum::{
    extract::{Path, Query, State},
    routing::{delete, get, post, put},
    Json, Router,
};

use crate::{
    dto::{
        common::{ApiResponse, PaginatedResponse, PaginationQuery},
        tool::{CreateToolRequest, ToolDto, UpdateToolRequest},
    },
    AppState,
};
use app::tool::{
    count_tools, create_tool, delete_tool, get_tool, list_available_tools,
    list_tools_by_owner, update_tool, CreateToolInput, UpdateToolInput,
};
use domain::member::MemberId;
use shared::{AppError, Id};

/// 工具路由
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", post(create_tool_handler))
        .route("/", get(list_tools_handler))
        .route("/:id", get(get_tool_handler))
        .route("/:id", put(update_tool_handler))
        .route("/:id", delete(delete_tool_handler))
        .route("/owner/:owner_id", get(list_tools_by_owner_handler))
}

/// 创建工具
/// 
/// TODO: 需要认证，从 JWT token 中获取 owner_id
#[cfg_attr(feature = "openapi", utoipa::path(post, path = "/api/v1/tools", tag = "tools"))]
async fn create_tool_handler(
    State(state): State<AppState>,
    Json(req): Json<CreateToolRequest>,
) -> Result<Json<ApiResponse<ToolDto>>, AppError> {
    // TODO: 从认证中间件获取当前用户 ID
    // 临时使用一个固定 ID 作为演示
    let owner_id = MemberId::new(); // 实际应该从 JWT Claims 中获取

    let input = CreateToolInput {
        owner_id,
        name: req.name,
        description: req.description,
        category: req.category,
        price_amount: req.price_amount,
        price_currency: req.price_currency,
    };

    let tool = create_tool(state.tool_repo.as_ref(), input).await?;
    let dto = ToolDto::from(&tool);

    Ok(Json(ApiResponse::success(dto)))
}

/// 获取工具详情
#[cfg_attr(feature = "openapi", utoipa::path(get, path = "/api/v1/tools/{id}", tag = "tools"))]
async fn get_tool_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<ToolDto>>, AppError> {
    let tool_id = parse_id(&id, "无效的工具 ID")?;

    let tool = get_tool(state.tool_repo.as_ref(), tool_id).await?;
    let dto = ToolDto::from(&tool);

    Ok(Json(ApiResponse::success(dto)))
}

/// 列出可用工具（分页）
#[cfg_attr(feature = "openapi", utoipa::path(get, path = "/api/v1/tools", tag = "tools"))]
async fn list_tools_handler(
    State(state): State<AppState>,
    Query(pagination): Query<PaginationQuery>,
) -> Result<Json<ApiResponse<PaginatedResponse<ToolDto>>>, AppError> {
    let tools = list_available_tools(
        state.tool_repo.as_ref(),
        pagination.page,
        pagination.page_size,
    )
    .await?;

    let total = count_tools(state.tool_repo.as_ref()).await?;

    let dtos: Vec<ToolDto> = tools.iter().map(ToolDto::from).collect();
    let response = PaginatedResponse::new(dtos, total, pagination.page, pagination.page_size);

    Ok(Json(ApiResponse::success(response)))
}

/// 更新工具
/// 
/// TODO: 需要认证，验证当前用户是工具的所有者
#[cfg_attr(feature = "openapi", utoipa::path(put, path = "/api/v1/tools/{id}", tag = "tools"))]
async fn update_tool_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<UpdateToolRequest>,
) -> Result<Json<ApiResponse<ToolDto>>, AppError> {
    let tool_id = parse_id(&id, "无效的工具 ID")?;

    // TODO: 从认证中间件获取当前用户 ID
    let requester_id = MemberId::new(); // 实际应该从 JWT Claims 中获取

    let input = UpdateToolInput {
        tool_id,
        requester_id,
        name: req.name,
        description: req.description,
        category: req.category,
        price_amount: req.price_amount,
        price_currency: req.price_currency,
    };

    let tool = update_tool(state.tool_repo.as_ref(), input).await?;
    let dto = ToolDto::from(&tool);

    Ok(Json(ApiResponse::success(dto)))
}

/// 删除工具
/// 
/// TODO: 需要认证，验证当前用户是工具的所有者
#[cfg_attr(feature = "openapi", utoipa::path(delete, path = "/api/v1/tools/{id}", tag = "tools"))]
async fn delete_tool_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    let tool_id = parse_id(&id, "无效的工具 ID")?;

    // TODO: 从认证中间件获取当前用户 ID
    let requester_id = MemberId::new();

    delete_tool(state.tool_repo.as_ref(), tool_id, requester_id).await?;

    Ok(Json(ApiResponse::success(())))
}

/// 列出指定所有者的工具
#[cfg_attr(feature = "openapi", utoipa::path(get, path = "/api/v1/tools/owner/{owner_id}", tag = "tools"))]
async fn list_tools_by_owner_handler(
    State(state): State<AppState>,
    Path(owner_id): Path<String>,
    Query(pagination): Query<PaginationQuery>,
) -> Result<Json<ApiResponse<PaginatedResponse<ToolDto>>>, AppError> {
    let owner_id = parse_id(&owner_id, "无效的所有者 ID")?;

    let tools = list_tools_by_owner(
        state.tool_repo.as_ref(),
        owner_id,
        pagination.page,
        pagination.page_size,
    )
    .await?;

    let total = tools.len() as i64; // 简化实现，实际应该有专门的 count_by_owner

    let dtos: Vec<ToolDto> = tools.iter().map(ToolDto::from).collect();
    let response = PaginatedResponse::new(dtos, total, pagination.page, pagination.page_size);

    Ok(Json(ApiResponse::success(response)))
}

// 辅助函数：ID 解析
fn parse_id<T>(id_str: &str, error_msg: &str) -> Result<shared::Id<T>, AppError> {
    Id::from_string(id_str).map_err(|_| AppError::validation(error_msg))
}
