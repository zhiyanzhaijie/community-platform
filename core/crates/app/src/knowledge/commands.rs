//! 知识命令（写操作）

use domain::{
    member::MemberId,
    knowledge::{Knowledge, KnowledgeForm, KnowledgeId, KnowledgeRepository},
};
use serde_json::Value;
use shared::{AppError, Result};
use tracing::instrument;

/// 创建知识输入
pub struct CreateKnowledgeInput {
    pub owner_id: MemberId,
    pub title: String,
    pub summary: Option<String>,
    pub category: String,
    pub form: KnowledgeForm,
    pub content: Value,
}

/// 创建知识
#[instrument(
    name = "create_knowledge",
    skip(repo, input),
    fields(
        owner_id = %input.owner_id,
        title = %input.title
    )
)]
pub async fn create_knowledge(
    repo: &dyn KnowledgeRepository,
    input: CreateKnowledgeInput,
) -> Result<Knowledge> {
    tracing::info!("开始创建知识");

    let knowledge = Knowledge::new(
        input.owner_id,
        input.title,
        input.summary,
        input.category,
        input.form,
        input.content,
    );

    repo.save(&knowledge).await?;

    tracing::info!(knowledge_id = %knowledge.id, "知识创建成功");
    Ok(knowledge)
}

/// 更新知识输入
pub struct UpdateKnowledgeInput {
    pub knowledge_id: KnowledgeId,
    pub requester_id: MemberId,
    pub title: Option<String>,
    /// Some(Some(x)) = 更新为 x，Some(None) = 置空，None = 不变
    pub summary: Option<Option<String>>,
    pub category: Option<String>,
    pub form: Option<KnowledgeForm>,
    pub content: Option<Value>,
}

/// 更新知识
#[instrument(
    name = "update_knowledge",
    skip(repo, input),
    fields(
        knowledge_id = %input.knowledge_id,
        requester_id = %input.requester_id
    )
)]
pub async fn update_knowledge(
    repo: &dyn KnowledgeRepository,
    input: UpdateKnowledgeInput,
) -> Result<Knowledge> {
    tracing::info!("开始更新知识");

    let mut knowledge = repo
        .find_by_id(input.knowledge_id)
        .await?
        .ok_or_else(|| AppError::not_found("知识不存在"))?;

    // 只有所有者可以更新
    if knowledge.owner_id != input.requester_id {
        return Err(AppError::Forbidden);
    }

    knowledge.update(
        input.title,
        input.summary,
        input.category,
        input.form,
        input.content,
    );

    repo.update(&knowledge).await?;

    tracing::info!(knowledge_id = %knowledge.id, "知识更新成功");
    Ok(knowledge)
}

/// 删除知识
#[instrument(
    name = "delete_knowledge",
    skip(repo),
    fields(knowledge_id = %knowledge_id, requester_id = %requester_id)
)]
pub async fn delete_knowledge(
    repo: &dyn KnowledgeRepository,
    knowledge_id: KnowledgeId,
    requester_id: MemberId,
) -> Result<()> {
    tracing::info!("开始删除知识");

    let knowledge = repo
        .find_by_id(knowledge_id)
        .await?
        .ok_or_else(|| AppError::not_found("知识不存在"))?;

    if knowledge.owner_id != requester_id {
        return Err(AppError::Forbidden);
    }

    repo.delete(knowledge_id).await?;

    tracing::info!(knowledge_id = %knowledge_id, "知识删除成功");
    Ok(())
}
