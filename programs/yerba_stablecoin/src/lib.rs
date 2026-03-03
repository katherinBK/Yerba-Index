pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("wallet");

#[program]
pub mod yerba_stablecoin {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }
    pub fn update_index(ctx: Context<UpdateIndex>, new_index: u64) -> Result<()> {
        update_index::handler(ctx, new_index)
    }
    pub fn mint_token(ctx: Context<MintToken>, amount: u64) -> Result<()> {
        mint::handler(ctx, amount)
    }
    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        stake::handler(ctx, amount)
    }
}