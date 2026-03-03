use anchor_lang::prelude::*;
use crate::state::StablecoinState;

#[derive(Accounts)]
pub struct MintToken<'info> {
    #[account(mut)]
    pub state: Account<'info, StablecoinState>,
    #[account(mut)]
    pub user: Signer<'info>,
    // agregar la cuenta del token asociado al usuario
}

pub fn handler(ctx: Context<MintToken>, amount: u64) -> Result<()> {
    let state = &mut ctx.accounts.state;
    state.total_supply = state.total_supply.checked_add(amount).unwrap();
    Ok(())
}