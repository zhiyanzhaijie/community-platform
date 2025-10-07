//! Tool 实体

use super::{Money, ToolId, ToolStatus};
use crate::member::MemberId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 工具聚合根
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    pub id: ToolId,
    pub owner_id: MemberId,
    pub name: String,
    pub description: Option<String>,
    pub category: String,
    pub price: Money,
    pub status: ToolStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Tool {
    /// 创建新工具
    pub fn new(
        owner_id: MemberId,
        name: String,
        description: Option<String>,
        category: String,
        price: Money,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: ToolId::new(),
            owner_id,
            name,
            description,
            category,
            price,
            status: ToolStatus::default(),
            created_at: now,
            updated_at: now,
        }
    }

    /// 更新工具信息
    pub fn update(
        &mut self,
        name: Option<String>,
        description: Option<String>,
        category: Option<String>,
        price: Option<Money>,
    ) {
        if let Some(name) = name {
            self.name = name;
        }
        if let Some(desc) = description {
            self.description = Some(desc);
        }
        if let Some(cat) = category {
            self.category = cat;
        }
        if let Some(price) = price {
            self.price = price;
        }
        self.updated_at = Utc::now();
    }

    /// 标记为已出租
    pub fn rent(&mut self) {
        self.status = ToolStatus::Rented;
        self.updated_at = Utc::now();
    }

    /// 标记为可用
    pub fn make_available(&mut self) {
        self.status = ToolStatus::Available;
        self.updated_at = Utc::now();
    }

    /// 标记为不可用
    pub fn make_unavailable(&mut self) {
        self.status = ToolStatus::Unavailable;
        self.updated_at = Utc::now();
    }

    /// 检查是否可用
    pub fn is_available(&self) -> bool {
        self.status == ToolStatus::Available
    }
}
