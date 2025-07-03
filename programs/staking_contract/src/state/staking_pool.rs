use anchor_lang::prelude::*;

#[account]
pub struct StakingPool {
    pub authority: Pubkey,
    pub mint: Pubkey,
    pub vault: Pubkey,
    pub total_staked: u64,
    pub reward_rate: u64, // reward per token per second
    pub bump: u8,
}