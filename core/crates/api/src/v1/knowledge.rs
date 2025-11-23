//! 知识 API 端点

use axum::{
    extract::{Path, Query, State},
    routing::{delete, get, post, put},
    Json, Router,
};

use crate::{
    dto::{
        common::{ApiResponse, PaginatedResponse, PaginationQuery},
        knowledge::{CreateKnowledgeRequest, KnowledgeDto, UpdateKnowledgeRequest},
    },
    middleware::auth::CurrentUser,
    AppState,
};
use app::knowledge::{
    count_knowledge, create_knowledge, delete_knowledge, get_knowledge,
    list_knowledge, list_knowledge_by_owner, update_knowledge,
    CreateKnowledgeInput, UpdateKnowledgeInput,
};
use domain::knowledge::KnowledgeForm;
use shared::{AppError, Id};
use std::str::FromStr;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", post(create_knowledge_handler))
        .route("/", get(list_knowledge_handler))
        .route("/:id", get(get_knowledge_handler))
        .route("/:id", put(update_knowledge_handler))
        .route("/:id", delete(delete_knowledge_handler))
        .route("/owner/:owner_id", get(list_knowledge_by_owner_handler))
}

#[utoipa::path(
    post,
    path = "/api/v1/knowledge",
    tag = "knowledge",
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn create_knowledge_handler(
    State(state): State<AppState>,
    CurrentUser(owner_id): CurrentUser,
    Json(req): Json<CreateKnowledgeRequest>,
) -> Result<Json<ApiResponse<KnowledgeDto>>, AppError> {
    let form = KnowledgeForm::from_str(&req.form)?;

    let input = CreateKnowledgeInput {
        owner_id,
        title: req.title,
        summary: req.summary,
        category: req.category,
        form,
        content: req.content,
    };

    let knowledge = create_knowledge(state.knowledge_repo.as_ref(), input).await?;
    let dto = KnowledgeDto::from(&knowledge);

    Ok(Json(ApiResponse::success(dto)))
}

#[utoipa::path(
    get,
    path = "/api/v1/knowledge/{id}",
    tag = "knowledge",
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_knowledge_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<KnowledgeDto>>, AppError> {
    let knowledge_id = parse_id(&id, "无效的知识 ID")?;

    let knowledge = get_knowledge(state.knowledge_repo.as_ref(), knowledge_id).await?;
    let dto = KnowledgeDto::from(&knowledge);

    Ok(Json(ApiResponse::success(dto)))
}

#[utoipa::path(
    get,
    path = "/api/v1/knowledge",
    tag = "knowledge",
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn list_knowledge_handler(
    State(state): State<AppState>,
    Query(pagination): Query<PaginationQuery>,
) -> Result<Json<ApiResponse<PaginatedResponse<KnowledgeDto>>>, AppError> {
    let knowledge_list = list_knowledge(
        state.knowledge_repo.as_ref(),
        pagination.page,
        pagination.page_size,
    )
    .await?;

    let total = count_knowledge(state.knowledge_repo.as_ref()).await?;

    let dtos: Vec<KnowledgeDto> = knowledge_list.iter().map(KnowledgeDto::from).collect();
    let response = PaginatedResponse::new(dtos, total, pagination.page, pagination.page_size);

    Ok(Json(ApiResponse::success(response)))
}

#[utoipa::path(
    put,
    path = "/api/v1/knowledge/{id}",
    tag = "knowledge",
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn update_knowledge_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    CurrentUser(requester_id): CurrentUser,
    Json(req): Json<UpdateKnowledgeRequest>,
) -> Result<Json<ApiResponse<KnowledgeDto>>, AppError> {
    let knowledge_id = parse_id(&id, "无效的知识 ID")?;

    let form = match req.form {
        Some(ref form_str) => Some(KnowledgeForm::from_str(form_str)?),
        None => None,
    };

    let input = UpdateKnowledgeInput {
        knowledge_id,
        requester_id,
        title: req.title,
        summary: req.summary,
        category: req.category,
        form,
        content: req.content,
    };

    let knowledge = update_knowledge(state.knowledge_repo.as_ref(), input).await?;
    let dto = KnowledgeDto::from(&knowledge);

    Ok(Json(ApiResponse::success(dto)))
}

#[utoipa::path(
    delete,
    path = "/api/v1/knowledge/{id}",
    tag = "knowledge",
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn delete_knowledge_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    CurrentUser(requester_id): CurrentUser,
) -> Result<Json<ApiResponse<()>>, AppError> {
    let knowledge_id = parse_id(&id, "无效的知识 ID")?;

    delete_knowledge(state.knowledge_repo.as_ref(), knowledge_id, requester_id).await?;

    Ok(Json(ApiResponse::success(())))
}

#[utoipa::path(
    get,
    path = "/api/v1/knowledge/owner/{owner_id}",
    tag = "knowledge",
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn list_knowledge_by_owner_handler(
    State(state): State<AppState>,
    Path(owner_id): Path<String>,
    Query(pagination): Query<PaginationQuery>,
) -> Result<Json<ApiResponse<PaginatedResponse<KnowledgeDto>>>, AppError> {
    let owner_id = parse_id(&owner_id, "无效的所有者 ID")?;

    let knowledge_list = list_knowledge_by_owner(
        state.knowledge_repo.as_ref(),
        owner_id,
        pagination.page,
        pagination.page_size,
    )
    .await?;

    let total = knowledge_list.len() as i64;

    let dtos: Vec<KnowledgeDto> = knowledge_list.iter().map(KnowledgeDto::from).collect();
    let response = PaginatedResponse::new(dtos, total, pagination.page, pagination.page_size);

    Ok(Json(ApiResponse::success(response)))
}

fn parse_id<T>(id_str: &str, error_msg: &str) -> Result<shared::Id<T>, AppError> {
    Id::from_string(id_str).map_err(|_| AppError::validation(error_msg))
}
