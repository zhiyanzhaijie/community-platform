//! Service实体

use super::ServiceId;
use crate::isu::ISU;
use crate::member::MemberId;
use crate::profession::ProfessionType;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use shared::{AppError, Result};

/// 服务聚合根
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Service {
    pub id: ServiceId,
    pub provider_id: MemberId,
    pub profession_type: ProfessionType,
    pub title: String,
    pub description: String,
    pub estimated_hours: Decimal,
    pub total_isu: ISU,
    pub status: ServiceStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 服务状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ServiceStatus {
    Available,    // 可用
    InProgress,   // 进行中
    Completed,    // 已完成
    Cancelled,    // 已取消
}

impl Default for ServiceStatus {
    fn default() -> Self {
        Self::Available
    }
}

impl std::fmt::Display for ServiceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Available => write!(f, "available"),
            Self::InProgress => write!(f, "in_progress"),
            Self::Completed => write!(f, "completed"),
            Self::Cancelled => write!(f, "cancelled"),
        }
    }
}

impl Service {
    /// 创建新服务
    pub fn new(
        provider_id: MemberId,
        profession_type: ProfessionType,
        title: String,
        description: String,
        estimated_hours: Decimal,
    ) -> Result<Self> {
        // 验证输入
        if title.trim().is_empty() {
            return Err(AppError::validation("服务标题不能为空"));
        }
        
        if estimated_hours <= Decimal::ZERO {
            return Err(AppError::validation("预估时长必须大于0"));
        }

        // 计算总ISU（使用职业默认费率）
        let profession_standard = crate::profession::ProfessionStandard::new_default(profession_type)?;
        let total_isu = profession_standard.calculate_total_isu(estimated_hours)?;

        let now = Utc::now();
        Ok(Self {
            id: ServiceId::new(),
            provider_id,
            profession_type,
            title: title.trim().to_string(),
            description: description.trim().to_string(),
            estimated_hours,
            total_isu,
            status: ServiceStatus::default(),
            created_at: now,
            updated_at: now,
        })
    }

    /// 更新服务信息
    pub fn update(
        &mut self,
        title: Option<String>,
        description: Option<String>,
        estimated_hours: Option<Decimal>,
    ) -> Result<()> {
        let mut updated = false;

        if let Some(new_title) = title {
            let trimmed_title = new_title.trim();
            if trimmed_title.is_empty() {
                return Err(AppError::validation("服务标题不能为空"));
            }
            self.title = trimmed_title.to_string();
            updated = true;
        }

        if let Some(new_description) = description {
            self.description = new_description.trim().to_string();
            updated = true;
        }

        if let Some(new_hours) = estimated_hours {
            if new_hours <= Decimal::ZERO {
                return Err(AppError::validation("预估时长必须大于0"));
            }
            self.estimated_hours = new_hours;
            
            // 重新计算总ISU
            let profession_standard = crate::profession::ProfessionStandard::new_default(self.profession_type)?;
            self.total_isu = profession_standard.calculate_total_isu(new_hours)?;
            updated = true;
        }

        if updated {
            self.updated_at = Utc::now();
        }

        Ok(())
    }

    /// 开始服务
    pub fn start(&mut self) -> Result<()> {
        match self.status {
            ServiceStatus::Available => {
                self.status = ServiceStatus::InProgress;
                self.updated_at = Utc::now();
                Ok(())
            }
            _ => Err(AppError::validation("只有可用状态的服务才能开始")),
        }
    }

    /// 完成服务
    pub fn complete(&mut self) -> Result<()> {
        match self.status {
            ServiceStatus::InProgress => {
                self.status = ServiceStatus::Completed;
                self.updated_at = Utc::now();
                Ok(())
            }
            _ => Err(AppError::validation("只有进行中的服务才能完成")),
        }
    }

    /// 取消服务
    pub fn cancel(&mut self) -> Result<()> {
        match self.status {
            ServiceStatus::Available | ServiceStatus::InProgress => {
                self.status = ServiceStatus::Cancelled;
                self.updated_at = Utc::now();
                Ok(())
            }
            _ => Err(AppError::validation("已完成的服务不能取消")),
        }
    }

    /// 检查是否可用
    pub fn is_available(&self) -> bool {
        self.status == ServiceStatus::Available
    }

    /// 检查服务是否属于指定提供者
    pub fn is_owned_by(&self, provider_id: &MemberId) -> bool {
        &self.provider_id == provider_id
    }
}