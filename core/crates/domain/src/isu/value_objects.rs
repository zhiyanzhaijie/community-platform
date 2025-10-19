//! ISU值对象

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use shared::{AppError, Result};

/// 国际标准单位（International Standard Unit）
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ISU(Decimal);

impl ISU {
    /// 创建新的ISU，确保非负
    pub fn new(amount: Decimal) -> Result<Self> {
        if amount < Decimal::ZERO {
            return Err(AppError::validation("ISU金额不能为负数"));
        }
        Ok(Self(amount))
    }

    /// 从浮点数创建ISU
    pub fn from_f64(amount: f64) -> Result<Self> {
        let decimal = Decimal::try_from(amount)
            .map_err(|_| AppError::validation("无效的ISU金额"))?;
        Self::new(decimal)
    }

    /// 获取ISU数值
    pub fn value(&self) -> Decimal {
        self.0
    }

    /// 转换为浮点数（用于计算）
    pub fn to_f64(&self) -> f64 {
        self.0.try_into().unwrap_or(0.0)
    }

    /// ISU加法
    pub fn add(&self, other: &ISU) -> Result<ISU> {
        Self::new(self.0 + other.0)
    }

    /// ISU减法
    pub fn subtract(&self, other: &ISU) -> Result<ISU> {
        if self.0 < other.0 {
            return Err(AppError::validation("ISU余额不足"));
        }
        Self::new(self.0 - other.0)
    }

    /// 乘以倍数
    pub fn multiply(&self, multiplier: Decimal) -> Result<ISU> {
        if multiplier < Decimal::ZERO {
            return Err(AppError::validation("乘数不能为负数"));
        }
        Self::new(self.0 * multiplier)
    }
}

impl Default for ISU {
    fn default() -> Self {
        Self(Decimal::ZERO)
    }
}

impl std::fmt::Display for ISU {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ISU", self.0)
    }
}

/// ISU费率（每小时ISU）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ISURate(Decimal);

impl ISURate {
    /// 创建新的ISU费率
    pub fn new(rate: Decimal) -> Result<Self> {
        if rate < Decimal::ZERO {
            return Err(AppError::validation("ISU费率不能为负数"));
        }
        Ok(Self(rate))
    }

    /// 从浮点数创建费率
    pub fn from_f64(rate: f64) -> Result<Self> {
        let decimal = Decimal::try_from(rate)
            .map_err(|_| AppError::validation("无效的ISU费率"))?;
        Self::new(decimal)
    }

    /// 获取费率值
    pub fn value(&self) -> Decimal {
        self.0
    }

    /// 根据时长计算ISU总额
    pub fn calculate_total(&self, hours: Decimal) -> Result<ISU> {
        ISU::new(self.0 * hours)
    }
}

impl std::fmt::Display for ISURate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ISU/hour", self.0)
    }
}