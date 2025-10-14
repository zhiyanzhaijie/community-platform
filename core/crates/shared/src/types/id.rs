//! 类型安全的 ID

use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
use uuid::Uuid;

/// 类型安全的 ID，使用幻影类型避免 ID 混用
#[derive(Debug, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Id<T> {
    value: Uuid,
    #[serde(skip)]
    _marker: PhantomData<T>,
}

impl<T> Id<T> {
    pub fn new() -> Self {
        Self {
            value: Uuid::new_v4(),
            _marker: PhantomData,
        }
    }

    pub fn from_uuid(value: Uuid) -> Self {
        Self {
            value,
            _marker: PhantomData,
        }
    }

    pub fn value(&self) -> Uuid {
        self.value
    }

    /// 从字符串解析 ID
    pub fn from_string(s: &str) -> Result<Self, uuid::Error> {
        let uuid = Uuid::parse_str(s)?;
        Ok(Self::from_uuid(uuid))
    }
}

impl<T> Default for Id<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> std::fmt::Display for Id<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

// 手动实现 PartialEq，不依赖 T 的 PartialEq
impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

// 手动实现 Eq
impl<T> Eq for Id<T> {}

// 手动实现 Clone，不依赖 T 的 Clone
impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        *self
    }
}

// 手动实现 Copy，不依赖 T 的 Copy
// PhantomData<T> 本身是 Copy 的，Uuid 也是 Copy 的
impl<T> Copy for Id<T> {}
