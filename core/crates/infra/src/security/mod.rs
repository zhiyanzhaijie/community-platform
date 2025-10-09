//! 安全相关服务

mod password_hasher;

pub use password_hasher::{Argon2PasswordHasher, PasswordHasher};
