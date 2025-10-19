//! ISU账户实体

use super::{ISU, ISUAccountId};
use crate::member::MemberId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use shared::{AppError, Result};

/// ISU账户聚合根
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ISUAccount {
    pub id: ISUAccountId,
    pub owner_id: MemberId,
    pub balance: ISU,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ISUAccount {
    /// 创建新的ISU账户
    pub fn new(owner_id: MemberId, initial_balance: ISU) -> Self {
        let now = Utc::now();
        Self {
            id: ISUAccountId::new(),
            owner_id,
            balance: initial_balance,
            created_at: now,
            updated_at: now,
        }
    }

    /// 存入ISU
    pub fn deposit(&mut self, amount: &ISU) -> Result<()> {
        self.balance = self.balance.add(amount)?;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// 提取ISU（检查余额）
    pub fn withdraw(&mut self, amount: &ISU) -> Result<()> {
        self.balance = self.balance.subtract(amount)?;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// 检查是否有足够余额
    pub fn has_sufficient_balance(&self, amount: &ISU) -> bool {
        self.balance.value() >= amount.value()
    }

    /// 获取当前余额
    pub fn get_balance(&self) -> ISU {
        self.balance
    }
}

/// ISU交易记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ISUTransaction {
    pub id: String, // 简化为字符串ID
    pub from_account_id: ISUAccountId,
    pub to_account_id: ISUAccountId,
    pub amount: ISU,
    pub transaction_type: ISUTransactionType,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// ISU交易类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ISUTransactionType {
    ServicePayment,    // 服务支付
    ToolRental,       // 工具租用
    InitialBalance,   // 初始余额
    AdminAdjustment,  // 管理员调整
}