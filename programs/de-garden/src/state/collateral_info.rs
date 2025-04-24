use anchor_lang::prelude::*;


// TODO: delete?
#[account]
#[derive(InitSpace)]
pub struct CollateralInfo {
  pub sensor: Pubkey, 
  pub collateralized_amount: u64,
  pub collateralized_at: i64,
  pub bump: u8,
}
