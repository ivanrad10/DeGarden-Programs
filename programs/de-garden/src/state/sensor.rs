use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq)]
pub enum SensorStatus {
    Uncollateralized,
    Collateralized,
    Slashed,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq)]
pub enum SensorType {
    Moisture,
    Flowmeter
}

#[account]
#[derive(InitSpace)]
pub struct Sensor {
    pub id: u64,
    pub model: SensorType,
    pub host: Pubkey,
    pub latitude: i64,
    pub longitude: i64,
    pub status: SensorStatus,
    pub last_collateralized_at: i64,
    pub last_uncollateralized_at: i64,
    pub last_slashed_at: i64,
    pub bump: u8,
}
