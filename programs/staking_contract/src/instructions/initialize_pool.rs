use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{Mint, Token, TokenAccount}};
use crate::state::StakingPool;

#[derive(Accounts)]
pub struct InitializePool<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    pub mint: Account<'info, Mint>,
    #[account(
        init,
        payer = authority,
        seeds = [b"staking_pool", authority.key().as_ref()],
        bump,
        space = 8 + std::mem::size_of::<StakingPool>(),
    )]
    pub pool: Account<'info, StakingPool>,
    #[account(
        init,
        payer = authority,
        associated_token::mint = mint,
        associated_token::authority = pool,
    )]
    pub vault: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> InitializePool<'info> {
    pub fn initialize(&mut self, reward_rate: u64, bumps: &InitializePoolBumps) -> Result<()> {
        self.pool.set_inner(StakingPool {
            authority: self.authority.key(),
            mint: self.mint.key(),
            vault: self.vault.key(),
            total_staked: 0,
            reward_rate,
            bump: bumps.pool,
        });
        Ok(())
    }
}