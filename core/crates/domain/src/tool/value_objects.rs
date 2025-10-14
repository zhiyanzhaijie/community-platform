//! 工具值对象

use serde::{Deserialize, Serialize};
use shared::{AppError, Result};

/// 金额
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Money {
    pub amount: i64,      // 以分为单位
    pub currency: Currency,
}

impl Money {
    pub fn new(amount: i64, currency: Currency) -> Result<Self> {
        if amount < 0 {
            return Err(AppError::validation("金额不能为负数"));
        }
        Ok(Self { amount, currency })
    }

    pub fn cny(amount: i64) -> Result<Self> {
        Self::new(amount, Currency::CNY)
    }
}

/// 货币类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Currency {
    CNY,
    USD,
}

impl Currency {
    /// 从字符串解析货币类型，默认为 CNY
    pub fn from_str(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "USD" => Self::USD,
            _ => Self::CNY, // 默认人民币
        }
    }
}

/// 工具状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ToolStatus {
    Available,
    Rented,
    Unavailable,
}

impl Default for ToolStatus {
    fn default() -> Self {
        Self::Available
    }
}

impl std::fmt::Display for ToolStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Available => write!(f, "available"),
            Self::Rented => write!(f, "rented"),
            Self::Unavailable => write!(f, "unavailable"),
        }
    }
}
