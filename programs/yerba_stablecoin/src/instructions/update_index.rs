use anchor_lang::prelude::*;
use crate::state::StablecoinState;

#[derive(Accounts)]
pub struct UpdateIndex<'info> {
    #[account(mut, has_one = admin)]
    pub state: Account<'info, StablecoinState>,
    pub admin: Signer<'info>,
}

pub fn handler(ctx: Context<UpdateIndex>, new_index: u64) -> Result<()> {
    let state = &mut ctx.accounts.state;
    state.index_value = new_index;
    Ok(())
}