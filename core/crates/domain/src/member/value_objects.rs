//! 会员值对象

use serde::{Deserialize, Serialize};
use shared::{AppError, Result};

/// 邮箱
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Email(String);

impl Email {
    pub fn new(value: impl Into<String>) -> Result<Self> {
        let value = value.into();
        if !value.contains('@') || value.len() < 3 {
            return Err(AppError::validation("无效的邮箱格式"));
        }
        Ok(Self(value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// 用户名
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Username(String);

impl Username {
    pub fn new(value: impl Into<String>) -> Result<Self> {
        let value = value.into();
        if value.len() < 3 || value.len() > 50 {
            return Err(AppError::validation("用户名长度必须在3-50之间"));
        }
        Ok(Self(value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Username {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// 会员状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MemberStatus {
    Active,
    Inactive,
    Banned,
}

impl Default for MemberStatus {
    fn default() -> Self {
        Self::Active
    }
}

impl std::fmt::Display for MemberStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Active => write!(f, "active"),
            Self::Inactive => write!(f, "inactive"),
            Self::Banned => write!(f, "banned"),
        }
    }
}

impl std::str::FromStr for MemberStatus {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "active" => Ok(Self::Active),
            "inactive" => Ok(Self::Inactive),
            "banned" => Ok(Self::Banned),
            _ => Err(AppError::validation(format!("无效的会员状态: {}", s))),
        }
    }
}
