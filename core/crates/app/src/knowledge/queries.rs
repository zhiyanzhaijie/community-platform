//! 知识查询（读操作）

use domain::{
    member::MemberId,
    knowledge::{Knowledge, KnowledgeId, KnowledgeRepository},
};
use shared::{AppError, Result};
use tracing::instrument;

/// 获取知识详情
#[instrument(name = "get_knowledge", skip(repo), fields(knowledge_id = %knowledge_id))]
pub async fn get_knowledge(
    repo: &dyn KnowledgeRepository,
    knowledge_id: KnowledgeId,
) -> Result<Knowledge> {
    tracing::info!("获取知识详情");

    let knowledge = repo
        .find_by_id(knowledge_id)
        .await?
        .ok_or_else(|| AppError::not_found("知识不存在"))?;

    Ok(knowledge)
}

/// 列出所有知识（分页）
#[instrument(name = "list_knowledge", skip(repo))]
pub async fn list_knowledge(
    repo: &dyn KnowledgeRepository,
    page: i64,
    page_size: i64,
) -> Result<Vec<Knowledge>> {
    tracing::info!("列出所有知识");
    repo.find_all(page, page_size).await
}

/// 列出某个用户的知识（分页）
#[instrument(name = "list_knowledge_by_owner", skip(repo), fields(owner_id = %owner_id))]
pub async fn list_knowledge_by_owner(
    repo: &dyn KnowledgeRepository,
    owner_id: MemberId,
    page: i64,
    page_size: i64,
) -> Result<Vec<Knowledge>> {
    tracing::info!("列出所有者的知识");
    repo.find_by_owner(owner_id, page, page_size).await
}

/// 获取知识总数
#[instrument(name = "count_knowledge", skip(repo))]
pub async fn count_knowledge(repo: &dyn KnowledgeRepository) -> Result<i64> {
    tracing::info!("统计知识总数");
    repo.count_all().await
}
