use anchor_lang::prelude::*;

use crate::{Sensor, SensorHost, SensorStatus, SENSOR_HOST_SEED, SENSOR_SEED};

pub fn register_sensor_handler(ctx: Context<RegisterSensor>, latitude: i64, longitude: i64) -> Result<()> {
    let sensor_host = &mut ctx.accounts.sensor_host_state;
    let sensor = &mut ctx.accounts.sensor;
    
    sensor.host = ctx.accounts.host.key();
    sensor.id = sensor_host.sensor_counter;
    sensor.status = SensorStatus::Uncollateralized;
    sensor.latitude = latitude;
    sensor.longitude = longitude;
    sensor.last_collateralized_at = 0;
    sensor.last_uncollateralized_at = 0;
    sensor.last_slashed_at = 0;
    sensor.total_income = 0;

    sensor_host.sensor_counter += 1;

    Ok(())
}

#[derive(Accounts)]
pub struct RegisterSensor<'info> {
    #[account(mut)]
    pub host: Signer<'info>,
    #[account(
        seeds = [SENSOR_HOST_SEED.as_bytes(), host.key().as_ref()],
        bump = sensor_host_state.bump
    )]
    pub sensor_host_state: Account<'info, SensorHost>,
    #[account(
        init,
        payer = host,
        space = 8 + Sensor::INIT_SPACE,
        seeds = [SENSOR_SEED.as_bytes(), sensor_host_state.key().as_ref(), &sensor_host_state.sensor_counter.to_le_bytes()],
        bump
    )]
    pub sensor: Account<'info, Sensor>,
    pub system_program: Program<'info, System>
}