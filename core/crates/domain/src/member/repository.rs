//! Member Repository trait

use super::{Email, Member, MemberId, Username};
use async_trait::async_trait;
use shared::Result;

/// 会员仓储接口
#[async_trait]
pub trait MemberRepository: Send + Sync {
    /// 保存会员
    async fn save(&self, member: &Member) -> Result<()>;

    /// 根据 ID 查找
    async fn find_by_id(&self, id: MemberId) -> Result<Option<Member>>;

    /// 根据邮箱查找
    async fn find_by_email(&self, email: &Email) -> Result<Option<Member>>;

    /// 根据用户名查找
    async fn find_by_username(&self, username: &Username) -> Result<Option<Member>>;

    /// 更新会员
    async fn update(&self, member: &Member) -> Result<()>;

    /// 删除会员
    async fn delete(&self, id: MemberId) -> Result<()>;
}
