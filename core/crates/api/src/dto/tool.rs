//! Tool DTOs

use domain::tool::{Currency, Money, Tool};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub struct CreateToolRequest {
    pub name: String,
    pub description: Option<String>,
    pub category: String,
    pub price_amount: i64,
    #[serde(default = "default_currency")]
    pub price_currency: String,
}

fn default_currency() -> String {
    "CNY".to_string()
}

#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub struct UpdateToolRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub category: Option<String>,
    pub price_amount: Option<i64>,
    pub price_currency: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ToolDto {
    pub id: String,
    pub owner_id: String,
    pub name: String,
    pub description: Option<String>,
    pub category: String,
    pub price: MoneyDto,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MoneyDto {
    pub amount: i64,
    pub currency: String,
}

impl From<&Tool> for ToolDto {
    fn from(tool: &Tool) -> Self {
        Self {
            id: tool.id.to_string(),
            owner_id: tool.owner_id.to_string(),
            name: tool.name.clone(),
            description: tool.description.clone(),
            category: tool.category.clone(),
            price: MoneyDto::from(&tool.price),
            status: tool.status.to_string(),
            created_at: tool.created_at.to_rfc3339(),
            updated_at: tool.updated_at.to_rfc3339(),
        }
    }
}

impl From<&Money> for MoneyDto {
    fn from(money: &Money) -> Self {
        let currency = match money.currency {
            Currency::CNY => "CNY",
            Currency::USD => "USD",
        };
        Self {
            amount: money.amount,
            currency: currency.to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ToolListResponse {
    pub tools: Vec<ToolDto>,
    pub total: i64,
    pub page: i64,
    pub page_size: i64,
}
