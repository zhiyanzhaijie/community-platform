//! 工具命令（写操作）

use domain::{
    member::MemberId,
    tool::{Currency, Money, Tool, ToolId, ToolRepository},
};
use shared::{AppError, Result};
use tracing::instrument;

/// 创建工具输入
pub struct CreateToolInput {
    pub owner_id: MemberId,
    pub name: String,
    pub description: Option<String>,
    pub category: String,
    pub price_amount: i64,
    pub price_currency: String,
}

/// 创建工具
#[instrument(
    name = "create_tool",
    skip(repo, input),
    fields(
        owner_id = %input.owner_id,
        name = %input.name
    )
)]
pub async fn create_tool(
    repo: &dyn ToolRepository,
    input: CreateToolInput,
) -> Result<Tool> {
    tracing::info!("开始创建工具");

    // 构建价格值对象
    let currency = parse_currency(&input.price_currency);
    let price = Money::new(input.price_amount, currency)?;

    // 创建工具
    let tool = Tool::new(
        input.owner_id,
        input.name,
        input.description,
        input.category,
        price,
    );

    // 保存到仓储
    repo.save(&tool).await?;

    tracing::info!(tool_id = %tool.id, "工具创建成功");
    Ok(tool)
}

/// 更新工具输入
pub struct UpdateToolInput {
    pub tool_id: ToolId,
    pub requester_id: MemberId,
    pub name: Option<String>,
    pub description: Option<String>,
    pub category: Option<String>,
    pub price_amount: Option<i64>,
    pub price_currency: Option<String>,
}

/// 更新工具
#[instrument(
    name = "update_tool",
    skip(repo, input),
    fields(
        tool_id = %input.tool_id,
        requester_id = %input.requester_id
    )
)]
pub async fn update_tool(
    repo: &dyn ToolRepository,
    input: UpdateToolInput,
) -> Result<Tool> {
    tracing::info!("开始更新工具");

    // 获取工具
    let mut tool = repo
        .find_by_id(input.tool_id)
        .await?
        .ok_or_else(|| AppError::not_found("工具不存在"))?;

    // 检查权限：只有所有者可以更新
    if tool.owner_id != input.requester_id {
        return Err(AppError::Forbidden);
    }

    // 构建价格值对象（如果提供）
    let price = if let (Some(amount), Some(currency)) = (input.price_amount, input.price_currency) {
        let currency = parse_currency(&currency);
        Some(Money::new(amount, currency)?)
    } else {
        None
    };

    // 更新工具
    tool.update(input.name, input.description, input.category, price);

    // 保存更新
    repo.update(&tool).await?;

    tracing::info!(tool_id = %tool.id, "工具更新成功");
    Ok(tool)
}

/// 删除工具
#[instrument(
    name = "delete_tool",
    skip(repo),
    fields(tool_id = %tool_id, requester_id = %requester_id)
)]
pub async fn delete_tool(
    repo: &dyn ToolRepository,
    tool_id: ToolId,
    requester_id: MemberId,
) -> Result<()> {
    tracing::info!("开始删除工具");

    // 获取工具
    let tool = repo
        .find_by_id(tool_id)
        .await?
        .ok_or_else(|| AppError::not_found("工具不存在"))?;

    // 检查权限：只有所有者可以删除
    if tool.owner_id != requester_id {
        return Err(AppError::Forbidden);
    }

    // 删除工具
    repo.delete(tool_id).await?;

    tracing::info!(tool_id = %tool_id, "工具删除成功");
    Ok(())
}

// 辅助函数：解析货币类型
fn parse_currency(currency: &str) -> Currency {
    match currency.to_uppercase().as_str() {
        "USD" => Currency::USD,
        _ => Currency::CNY, // 默认人民币
    }
}
