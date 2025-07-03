use anchor_lang::prelude::*;
use crate::{state::{StakingPool, UserStake}};
use crate::error::StakingError;

#[derive(Accounts)]
pub struct ClaimRewards<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub pool: Account<'info, StakingPool>,
    #[account(mut, seeds = [b"user_stake", pool.key().as_ref(), user.key().as_ref()], bump)]
    pub user_stake: Account<'info, UserStake>,
}

impl<'info> ClaimRewards<'info> {
    pub fn claim(&mut self) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;
        let rewards = self.user_stake.amount * (now - self.user_stake.last_staked_time) as u64 * self.pool.reward_rate;
        require!(rewards > 0, StakingError::NothingToClaim);
        self.user_stake.reward_debt += rewards;
        self.user_stake.last_staked_time = now;
        Ok(())
    }
}
