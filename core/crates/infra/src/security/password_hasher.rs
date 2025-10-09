//! 密码哈希服务

use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher as Argon2PasswordHasherTrait, PasswordVerifier,
};
use shared::{AppError, Result};

/// 密码哈希器 trait
pub trait PasswordHasher: Send + Sync {
    /// 哈希密码
    fn hash(&self, password: &str) -> Result<String>;
    
    /// 验证密码
    fn verify(&self, password: &str, hash: &str) -> Result<bool>;
}

/// Argon2 密码哈希实现
pub struct Argon2PasswordHasher;

impl Argon2PasswordHasher {
    pub fn new() -> Self {
        Self
    }
}

impl Default for Argon2PasswordHasher {
    fn default() -> Self {
        Self::new()
    }
}

impl PasswordHasher for Argon2PasswordHasher {
    fn hash(&self, password: &str) -> Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        argon2
            .hash_password(password.as_bytes(), &salt)
            .map(|hash| hash.to_string())
            .map_err(|e| AppError::internal(format!("密码哈希失败: {}", e)))
    }

    fn verify(&self, password: &str, hash: &str) -> Result<bool> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| AppError::internal(format!("密码哈希解析失败: {}", e)))?;

        let argon2 = Argon2::default();

        Ok(argon2
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_and_verify() {
        let hasher = Argon2PasswordHasher::new();
        let password = "test_password_123";

        // 哈希密码
        let hash = hasher.hash(password).unwrap();
        assert!(!hash.is_empty());

        // 验证正确密码
        assert!(hasher.verify(password, &hash).unwrap());

        // 验证错误密码
        assert!(!hasher.verify("wrong_password", &hash).unwrap());
    }
}
