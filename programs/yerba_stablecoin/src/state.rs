use anchor_lang::prelude::*;

#[account]
pub struct StablecoinState {
    pub admin: Pubkey,       
    pub index_value: u64,   
}