//! 职业标准管理相关用例

pub mod assign_decider;
pub mod update_profession_rate;

// 重导出
pub use assign_decider::{execute as assign_decider, revoke_decider, AssignDeciderInput, AssignDeciderOutput};
pub use update_profession_rate::{execute as update_profession_rate, UpdateProfessionRateInput, UpdateProfessionRateOutput};