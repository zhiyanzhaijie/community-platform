use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateToolRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub category: Option<String>,
    pub price_amount: Option<i64>,
    pub price_currency: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MoneyDto {
    pub amount: i64,
    pub currency: String,
}
