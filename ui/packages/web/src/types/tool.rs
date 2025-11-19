use serde::{Deserialize, Serialize};

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
