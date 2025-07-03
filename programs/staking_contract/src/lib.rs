use anchor_lang::prelude::*;

mod state;
mod instructions;
mod error;
mod constants;

use instructions::*;


declare_id!("H2TVseadWUma5YLjLNj9RrPQ44NAYvfpHz4PbMkFRy2E");

#[program]
pub mod staking_contract {
    use super::*;

    pub fn initialize_pool(ctx: Context<InitializePool>, reward_rate: u64) -> Result<()> {
        ctx.accounts.initialize(reward_rate, &ctx.bumps)?;
        Ok(())
    }

    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        ctx.accounts.stake(amount)?;
        Ok(())
    }

    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        ctx.accounts.unstake()?;
        Ok(())
    }

    pub fn claim_rewards(ctx: Context<ClaimRewards>) -> Result<()> {
        ctx.accounts.claim()?;
        Ok(())
    }
}
