use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Token, TokenAccount, Transfer};
use crate::{state::{StakingPool, UserStake}};
use crate::error::StakingError;

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub pool: Account<'info, StakingPool>,
    #[account(
        init_if_needed,
        payer = user,
        seeds = [b"user_stake", pool.key().as_ref(), user.key().as_ref()],
        bump,
        space = 8 + std::mem::size_of::<UserStake>(),
    )]
    pub user_stake: Account<'info, UserStake>,
    #[account(mut)]
    pub user_ata: Account<'info, TokenAccount>,
    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> Stake<'info> {
    pub fn stake(&mut self, amount: u64) -> Result<()> {
        require!(amount > 0, StakingError::InvalidAmount);
        let now = Clock::get()?.unix_timestamp;

        self.user_stake.reward_debt +=
            self.user_stake.amount * (now - self.user_stake.last_staked_time) as u64 * self.pool.reward_rate;
        self.user_stake.amount += amount;
        self.user_stake.last_staked_time = now;
        self.pool.total_staked += amount;

        let cpi_accounts = Transfer {
            from: self.user_ata.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.user.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);
        transfer(cpi_ctx, amount)?;
        Ok(())
    }
}