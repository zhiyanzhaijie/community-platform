//! Transaction实体

use super::TransactionId;
use crate::isu::ISU;
use crate::member::MemberId;
use crate::service::ServiceId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use shared::{AppError, Result};

/// 交易聚合根
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: TransactionId,
    pub buyer_id: MemberId,
    pub seller_id: MemberId,
    pub item_type: TransactionItemType,
    pub isu_amount: ISU,
    pub status: TransactionStatus,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

/// 交易项目类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type", content = "id")]
pub enum TransactionItemType {
    Service(ServiceId),
    Tool(crate::tool::ToolId),
}

/// 交易状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransactionStatus {
    Pending,    // 待确认
    Confirmed,  // 已确认
    InProgress, // 进行中
    Completed,  // 已完成
    Cancelled,  // 已取消
    Disputed,   // 争议中
}

impl Default for TransactionStatus {
    fn default() -> Self {
        Self::Pending
    }
}

impl std::fmt::Display for TransactionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pending => write!(f, "pending"),
            Self::Confirmed => write!(f, "confirmed"),
            Self::InProgress => write!(f, "in_progress"),
            Self::Completed => write!(f, "completed"),
            Self::Cancelled => write!(f, "cancelled"),
            Self::Disputed => write!(f, "disputed"),
        }
    }
}

impl Transaction {
    /// 创建新交易
    pub fn new(
        buyer_id: MemberId,
        seller_id: MemberId,
        item_type: TransactionItemType,
        isu_amount: ISU,
        description: Option<String>,
    ) -> Result<Self> {
        // 验证买家和卖家不能是同一人
        if buyer_id == seller_id {
            return Err(AppError::validation("买家和卖家不能是同一人"));
        }

        let now = Utc::now();
        Ok(Self {
            id: TransactionId::new(),
            buyer_id,
            seller_id,
            item_type,
            isu_amount,
            status: TransactionStatus::default(),
            description,
            created_at: now,
            updated_at: now,
            completed_at: None,
        })
    }

    /// 确认交易（卖家确认）
    pub fn confirm(&mut self) -> Result<()> {
        match self.status {
            TransactionStatus::Pending => {
                self.status = TransactionStatus::Confirmed;
                self.updated_at = Utc::now();
                Ok(())
            }
            _ => Err(AppError::validation("只有待确认状态的交易才能确认")),
        }
    }

    /// 开始交易
    pub fn start(&mut self) -> Result<()> {
        match self.status {
            TransactionStatus::Confirmed => {
                self.status = TransactionStatus::InProgress;
                self.updated_at = Utc::now();
                Ok(())
            }
            _ => Err(AppError::validation("只有已确认的交易才能开始")),
        }
    }

    /// 完成交易
    pub fn complete(&mut self) -> Result<()> {
        match self.status {
            TransactionStatus::InProgress => {
                self.status = TransactionStatus::Completed;
                self.updated_at = Utc::now();
                self.completed_at = Some(Utc::now());
                Ok(())
            }
            _ => Err(AppError::validation("只有进行中的交易才能完成")),
        }
    }

    /// 取消交易
    pub fn cancel(&mut self) -> Result<()> {
        match self.status {
            TransactionStatus::Pending | TransactionStatus::Confirmed => {
                self.status = TransactionStatus::Cancelled;
                self.updated_at = Utc::now();
                Ok(())
            }
            _ => Err(AppError::validation("进行中或已完成的交易不能取消")),
        }
    }

    /// 标记为争议
    pub fn dispute(&mut self) -> Result<()> {
        match self.status {
            TransactionStatus::InProgress => {
                self.status = TransactionStatus::Disputed;
                self.updated_at = Utc::now();
                Ok(())
            }
            _ => Err(AppError::validation("只有进行中的交易才能标记为争议")),
        }
    }

    /// 检查交易是否可以取消
    pub fn can_cancel(&self) -> bool {
        matches!(
            self.status,
            TransactionStatus::Pending | TransactionStatus::Confirmed
        )
    }

    /// 检查交易是否已完成
    pub fn is_completed(&self) -> bool {
        self.status == TransactionStatus::Completed
    }

    /// 检查用户是否是交易的参与者
    pub fn is_participant(&self, member_id: &MemberId) -> bool {
        &self.buyer_id == member_id || &self.seller_id == member_id
    }

    /// 检查用户是否是买家
    pub fn is_buyer(&self, member_id: &MemberId) -> bool {
        &self.buyer_id == member_id
    }

    /// 检查用户是否是卖家
    pub fn is_seller(&self, member_id: &MemberId) -> bool {
        &self.seller_id == member_id
    }

    /// 获取交易项目描述
    pub fn get_item_description(&self) -> String {
        match &self.item_type {
            TransactionItemType::Service(service_id) => {
                format!("服务交易 - Service ID: {}", service_id.value())
            }
            TransactionItemType::Tool(tool_id) => {
                format!("工具租用 - Tool ID: {}", tool_id.value())
            }
        }
    }
}