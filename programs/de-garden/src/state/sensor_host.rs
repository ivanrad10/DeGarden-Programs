use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
// TODO: add moisture and flowmeter sensor
pub struct SensorHost {
    pub address: Pubkey,
    pub moisture_sensor_counter: u64,
    pub flowmeter_sensor_counter: u64,
    pub bump: u8,
}
