//! 工具查询（读操作）

use domain::{
    member::MemberId,
    tool::{Tool, ToolId, ToolRepository},
};
use shared::{AppError, Result};
use tracing::instrument;

/// 获取工具详情
#[instrument(name = "get_tool", skip(repo), fields(tool_id = %tool_id))]
pub async fn get_tool(repo: &dyn ToolRepository, tool_id: ToolId) -> Result<Tool> {
    tracing::info!("获取工具详情");

    let tool = repo
        .find_by_id(tool_id)
        .await?
        .ok_or_else(|| AppError::not_found("工具不存在"))?;

    Ok(tool)
}

/// 列出可用工具（分页）
#[instrument(name = "list_available_tools", skip(repo))]
pub async fn list_available_tools(
    repo: &dyn ToolRepository,
    page: i64,
    page_size: i64,
) -> Result<Vec<Tool>> {
    tracing::info!("列出可用工具");
    repo.find_available(page, page_size).await
}

/// 列出所有者的工具（分页）
#[instrument(name = "list_tools_by_owner", skip(repo), fields(owner_id = %owner_id))]
pub async fn list_tools_by_owner(
    repo: &dyn ToolRepository,
    owner_id: MemberId,
    page: i64,
    page_size: i64,
) -> Result<Vec<Tool>> {
    tracing::info!("列出所有者的工具");
    repo.find_by_owner(owner_id, page, page_size).await
}

/// 获取工具总数
#[instrument(name = "count_tools", skip(repo))]
pub async fn count_tools(repo: &dyn ToolRepository) -> Result<i64> {
    tracing::info!("统计工具总数");
    repo.count().await
}
