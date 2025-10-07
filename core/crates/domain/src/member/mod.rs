//! 会员聚合根

mod entity;
mod repository;
mod value_objects;

pub use entity::Member;
pub use repository::MemberRepository;
pub use value_objects::{Email, MemberStatus, Username};

// 类型别名
pub type MemberId = shared::Id<Member>;
