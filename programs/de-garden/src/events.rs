use anchor_lang::prelude::*;

#[event]
pub struct PaySensorDataRequest {
    pub sensor_id: u64,
    pub payer: Pubkey,
}
