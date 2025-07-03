use anchor_lang::prelude::*;
use anchor_spl::token::{ transfer, Token, TokenAccount, Transfer };
use crate::{ state::{ StakingPool, UserStake } };
use crate::error::StakingError;

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub pool: Account<'info, StakingPool>,
    #[account(mut, seeds = [b"user_stake", pool.key().as_ref(), user.key().as_ref()], bump)]
    pub user_stake: Account<'info, UserStake>,
    #[account(mut)]
    pub user_ata: Account<'info, TokenAccount>,
    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

impl<'info> Unstake<'info> {
    pub fn unstake(&mut self) -> Result<()> {
        let amount = self.user_stake.amount;
        require!(amount > 0, StakingError::NoStake);

        self.pool.total_staked -= amount;
        self.user_stake.amount = 0;

        let signer_seeds: &[&[&[u8]]] = &[
            &[b"staking_pool", self.pool.authority.as_ref(), &[self.pool.bump]],
        ];
        
        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.user_ata.to_account_info(),
            authority: self.pool.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            cpi_accounts,
            signer_seeds
        );
        transfer(cpi_ctx, amount)?;
        Ok(())
    }
}
