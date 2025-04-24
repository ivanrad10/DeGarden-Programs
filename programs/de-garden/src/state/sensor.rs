use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq)]
pub enum SensorStatus {
    Uncollateralized,
    Collateralized,
    Slashed,
}

#[account]
#[derive(InitSpace)]
pub struct Sensor {
    pub id: u64,
    pub host: Pubkey,
    pub latitude: i64,
    pub longitude: i64,
    pub status: SensorStatus,
    pub last_collateralized_at: i64,
    pub last_uncollateralized_at: i64,
    pub last_slashed_at: i64,
    pub total_income: u64,
    pub bump: u8
}