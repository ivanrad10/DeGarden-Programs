use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct SensorHost {
    pub address: Pubkey,
    pub sensor_counter: u64,
    pub bump: u8,
}
