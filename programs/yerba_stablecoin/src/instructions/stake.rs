use anchor_lang::prelude::*;
use crate::state::StablecoinState;

#[account]
pub struct StakeAccount {
    pub owner: Pubkey,
    pub amount: u64,
    pub start_time: i64,
}

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(init, payer = user, space = 8 + 32 + 8 + 8)]
    pub stake_account: Account<'info, StakeAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Stake>, amount: u64) -> Result<()> {
    let stake_account = &mut ctx.accounts.stake_account;
    stake_account.owner = ctx.accounts.user.key();
    stake_account.amount = amount;
    stake_account.start_time = Clock::get()?.unix_timestamp;
    Ok(())
}