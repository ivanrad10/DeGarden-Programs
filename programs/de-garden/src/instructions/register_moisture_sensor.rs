use anchor_lang::prelude::*;

use crate::{Sensor, SensorHost, SensorStatus, SensorType, ANCHOR_DISCRIMINATOR, MOISTURE_SENSOR_SEED, SENSOR_HOST_SEED};

pub fn register_moisture_sensor_handler(
    ctx: Context<RegisterMoistureSensor>,
    latitude: i64,
    longitude: i64,
) -> Result<()> {
    let sensor_host = &mut ctx.accounts.sensor_host_state;
    let moisture_sensor = &mut ctx.accounts.moisture_sensor;

    moisture_sensor.bump = ctx.bumps.moisture_sensor;
    moisture_sensor.host = ctx.accounts.host.key();
    moisture_sensor.model = SensorType::Moisture;
    moisture_sensor.id = sensor_host.moisture_sensor_counter;
    moisture_sensor.status = SensorStatus::Uncollateralized;
    moisture_sensor.latitude = latitude;
    moisture_sensor.longitude = longitude;
    moisture_sensor.last_collateralized_at = 0;
    moisture_sensor.last_uncollateralized_at = 0;
    moisture_sensor.last_slashed_at = 0;

    sensor_host.moisture_sensor_counter += 1;

    Ok(())
}

#[derive(Accounts)]
pub struct RegisterMoistureSensor<'info> {
    #[account(mut)]
    pub host: Signer<'info>,
    #[account(
        mut,
        seeds = [SENSOR_HOST_SEED.as_bytes(), host.key().as_ref()],
        bump = sensor_host_state.bump
    )]
    pub sensor_host_state: Account<'info, SensorHost>,
    #[account(
        init,
        payer = host,
        space = ANCHOR_DISCRIMINATOR + Sensor::INIT_SPACE,
        seeds = [MOISTURE_SENSOR_SEED.as_bytes(), sensor_host_state.key().as_ref(), &sensor_host_state.moisture_sensor_counter.to_le_bytes()],
        bump
    )]
    pub moisture_sensor: Account<'info, Sensor>,
    pub system_program: Program<'info, System>,
}
