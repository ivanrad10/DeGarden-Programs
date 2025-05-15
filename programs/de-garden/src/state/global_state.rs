use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct GlobalState {
    pub token_price_in_lamports: u64,
    pub bump: u8,
}
