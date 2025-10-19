//! 职业标准管理实体

use super::{ProfessionStandardId, ProfessionType};
use crate::isu::ISURate;
use crate::member::MemberId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use shared::{AppError, Result};

/// 职业标准管理实体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfessionStandardEntity {
    pub id: ProfessionStandardId,
    pub profession_type: ProfessionType,
    pub isu_rate: ISURate,
    pub description: String,
    pub is_active: bool,
    pub created_by: MemberId,
    pub updated_by: MemberId,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ProfessionStandardEntity {
    /// 创建新的职业标准
    pub fn new(
        profession_type: ProfessionType,
        isu_rate: ISURate,
        description: String,
        creator_id: MemberId,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: ProfessionStandardId::new(),
            profession_type,
            isu_rate,
            description,
            is_active: true,
            created_by: creator_id,
            updated_by: creator_id,
            created_at: now,
            updated_at: now,
        }
    }

    /// 创建默认职业标准
    pub fn new_default(profession_type: ProfessionType, creator_id: MemberId) -> Result<Self> {
        let isu_rate = profession_type.default_rate()?;
        let description = format!("{}的默认标准费率", profession_type.display_name());
        
        Ok(Self::new(profession_type, isu_rate, description, creator_id))
    }

    /// 更新ISU费率
    pub fn update_rate(
        &mut self,
        new_rate: ISURate,
        description: Option<String>,
        updater_id: MemberId,
    ) -> Result<()> {
        self.isu_rate = new_rate;
        
        if let Some(desc) = description {
            if !desc.trim().is_empty() {
                self.description = desc.trim().to_string();
            }
        }
        
        self.updated_by = updater_id;
        self.updated_at = Utc::now();
        Ok(())
    }

    /// 激活标准
    pub fn activate(&mut self, updater_id: MemberId) {
        self.is_active = true;
        self.updated_by = updater_id;
        self.updated_at = Utc::now();
    }

    /// 停用标准
    pub fn deactivate(&mut self, updater_id: MemberId) {
        self.is_active = false;
        self.updated_by = updater_id;
        self.updated_at = Utc::now();
    }

    /// 检查是否可用
    pub fn is_available(&self) -> bool {
        self.is_active
    }

    /// 根据工时计算总ISU
    pub fn calculate_total_isu(&self, hours: rust_decimal::Decimal) -> Result<crate::isu::ISU> {
        if !self.is_active {
            return Err(AppError::validation("职业标准已停用"));
        }
        self.isu_rate.calculate_total(hours)
    }
}

/// 职业标准变更记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfessionStandardHistory {
    pub id: String, // 简化为字符串ID
    pub standard_id: ProfessionStandardId,
    pub action: StandardAction,
    pub old_rate: Option<ISURate>,
    pub new_rate: Option<ISURate>,
    pub reason: String,
    pub changed_by: MemberId,
    pub created_at: DateTime<Utc>,
}

/// 标准变更动作
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StandardAction {
    Created,     // 创建
    RateUpdated, // 费率更新
    Activated,   // 激活
    Deactivated, // 停用
}