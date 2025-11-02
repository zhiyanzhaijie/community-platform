use serde::{Deserialize, Serialize};

/// API 响应包装
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

/// 分页响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub total: i64,
    pub page: i64,
    pub page_size: i64,
    pub total_pages: i64,
}

/// 会员信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Member {
    pub id: String,
    pub email: String,
    pub username: String,
    pub status: String,
    pub created_at: String,
}

/// 注册请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RegisterRequest {
    pub email: String,
    pub username: String,
    pub password: String,
}

/// 登录请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

/// 登录响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LoginResponse {
    pub token: String,
    pub member: Member,
}

/// 工具信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Tool {
    pub id: String,
    pub owner_id: String,
    pub name: String,
    pub description: Option<String>,
    pub category: String,
    pub price: Money,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

/// 金额
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Money {
    pub amount: i64,
    pub currency: String,
}

/// 创建工具请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateToolRequest {
    pub name: String,
    pub description: Option<String>,
    pub category: String,
    pub price_amount: i64,
    pub price_currency: String,
}

/// 更新工具请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateToolRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub category: Option<String>,
    pub price_amount: Option<i64>,
    pub price_currency: Option<String>,
}
