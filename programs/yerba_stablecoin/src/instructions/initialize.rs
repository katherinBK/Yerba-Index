use anchor_lang::prelude::*;
use crate::state::StablecoinState;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 32 + 8 + 8)]
    pub state: Account<'info, StablecoinState>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Initialize>) -> Result<()> {
    let state = &mut ctx.accounts.state;
    state.admin = ctx.accounts.user.key();
    state.index_value = 0;
    state.total_supply = 0;
    Ok(())
}