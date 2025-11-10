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

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", post(create_tool_handler))
        .route("/", get(list_tools_handler))
        .route("/:id", get(get_tool_handler))
        .route("/:id", put(update_tool_handler))
        .route("/:id", delete(delete_tool_handler))
        .route("/owner/:owner_id", get(list_tools_by_owner_handler))
}

#[utoipa::path(post, path = "/api/v1/tools", tag = "tools")]
async fn create_tool_handler(
    State(state): State<AppState>,
    Json(req): Json<CreateToolRequest>,
) -> Result<Json<ApiResponse<ToolDto>>, AppError> {
    let owner_id = MemberId::new();

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

#[utoipa::path(get, path = "/api/v1/tools/{id}", tag = "tools")]
async fn get_tool_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<ToolDto>>, AppError> {
    let tool_id = parse_id(&id, "无效的工具 ID")?;

    let tool = get_tool(state.tool_repo.as_ref(), tool_id).await?;
    let dto = ToolDto::from(&tool);

    Ok(Json(ApiResponse::success(dto)))
}

#[utoipa::path(get, path = "/api/v1/tools", tag = "tools")]
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

#[utoipa::path(put, path = "/api/v1/tools/{id}", tag = "tools")]
async fn update_tool_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<UpdateToolRequest>,
) -> Result<Json<ApiResponse<ToolDto>>, AppError> {
    let tool_id = parse_id(&id, "无效的工具 ID")?;

    let requester_id = MemberId::new();

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

#[utoipa::path(delete, path = "/api/v1/tools/{id}", tag = "tools")]
async fn delete_tool_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    let tool_id = parse_id(&id, "无效的工具 ID")?;

    let requester_id = MemberId::new();

    delete_tool(state.tool_repo.as_ref(), tool_id, requester_id).await?;

    Ok(Json(ApiResponse::success(())))
}

#[utoipa::path(get, path = "/api/v1/tools/owner/{owner_id}", tag = "tools")]
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

    let total = tools.len() as i64;

    let dtos: Vec<ToolDto> = tools.iter().map(ToolDto::from).collect();
    let response = PaginatedResponse::new(dtos, total, pagination.page, pagination.page_size);

    Ok(Json(ApiResponse::success(response)))
}

fn parse_id<T>(id_str: &str, error_msg: &str) -> Result<shared::Id<T>, AppError> {
    Id::from_string(id_str).map_err(|_| AppError::validation(error_msg))
}
