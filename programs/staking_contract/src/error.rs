use anchor_lang::prelude::*;

#[error_code]
pub enum StakingError {
    #[msg("Invalid staking amount")] InvalidAmount,
    #[msg("No stake found")] NoStake,
    #[msg("Nothing to claim")] NothingToClaim,
}