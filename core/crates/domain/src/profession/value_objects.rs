//! 职业值对象和常量

use crate::isu::ISURate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use shared::{AppError, Result};

/// 职业类型（MVP版本硬编码5种）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProfessionType {
    Cleaning,      // 清洁服务
    BasicRepair,   // 基础维修
    HomeTutoring,  // 家庭教学
    Documentation, // 文档/翻译
    Cooking,       // 烹饪服务
}

impl ProfessionType {
    /// 获取职业的显示名称
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Cleaning => "清洁服务",
            Self::BasicRepair => "基础维修",
            Self::HomeTutoring => "家庭教学",
            Self::Documentation => "文档/翻译",
            Self::Cooking => "烹饪服务",
        }
    }

    /// 获取职业的默认ISU费率
    pub fn default_rate(&self) -> Result<ISURate> {
        let rate = match self {
            Self::Cleaning => 1.0,
            Self::BasicRepair => 1.5,
            Self::HomeTutoring => 2.0,
            Self::Documentation => 1.8,
            Self::Cooking => 1.2,
        };
        ISURate::from_f64(rate)
    }

    /// 获取所有职业类型
    pub fn all() -> Vec<ProfessionType> {
        vec![
            Self::Cleaning,
            Self::BasicRepair,
            Self::HomeTutoring,
            Self::Documentation,
            Self::Cooking,
        ]
    }

    /// 从字符串解析职业类型
    pub fn from_str(s: &str) -> Result<Self> {
        match s {
            "cleaning" => Ok(Self::Cleaning),
            "basic_repair" => Ok(Self::BasicRepair),
            "home_tutoring" => Ok(Self::HomeTutoring),
            "documentation" => Ok(Self::Documentation),
            "cooking" => Ok(Self::Cooking),
            _ => Err(AppError::validation(format!("未知的职业类型: {}", s))),
        }
    }
}

impl std::fmt::Display for ProfessionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Self::Cleaning => "cleaning",
            Self::BasicRepair => "basic_repair",
            Self::HomeTutoring => "home_tutoring",
            Self::Documentation => "documentation",
            Self::Cooking => "cooking",
        };
        write!(f, "{}", name)
    }
}

/// 职业标准（将费率与职业绑定）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfessionStandard {
    pub profession_type: ProfessionType,
    pub isu_rate: ISURate,
    pub description: String,
}

impl ProfessionStandard {
    /// 创建默认的职业标准
    pub fn new_default(profession_type: ProfessionType) -> Result<Self> {
        let isu_rate = profession_type.default_rate()?;
        let description = format!("{}的标准费率", profession_type.display_name());
        
        Ok(Self {
            profession_type,
            isu_rate,
            description,
        })
    }

    /// 创建自定义职业标准
    pub fn new_custom(
        profession_type: ProfessionType,
        isu_rate: ISURate,
        description: String,
    ) -> Self {
        Self {
            profession_type,
            isu_rate,
            description,
        }
    }

    /// 根据工时计算总ISU
    pub fn calculate_total_isu(&self, hours: Decimal) -> Result<crate::isu::ISU> {
        self.isu_rate.calculate_total(hours)
    }
}