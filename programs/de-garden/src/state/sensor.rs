use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq)]
pub enum SensorStatus {
    Uncollateralized,
    Collateralized,
    Slashed,
}

// TODO: check if we need collateral info or we can put all in once place
#[account]
#[derive(InitSpace)]
pub struct Sensor {
    pub id: u64,
    // host
    pub latitude: i64,
    pub longitude: i64,
    pub status: SensorStatus,
    pub last_collateralized_at: i64,
    pub last_uncollateralized_at: i64,
    pub last_slashed_at: i64,
    pub bump: u8
}