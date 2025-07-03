use anchor_lang::prelude::*;

#[account]
pub struct UserStake {
    pub amount: u64,
    pub reward_debt: u64,
    pub last_staked_time: i64,
}