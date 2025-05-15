use anchor_lang::prelude::*;

use crate::{Sensor, SensorHost, SensorStatus, SensorType, ANCHOR_DISCRIMINATOR, FLOWMETER_SENSOR_SEED, SENSOR_HOST_SEED};

pub fn register_flowmeter_sensor_handler(
    ctx: Context<RegisterFlowmeterSensor>,
    latitude: i64,
    longitude: i64,
) -> Result<()> {
    let sensor_host = &mut ctx.accounts.sensor_host_state;
    let flowmeter_sensor = &mut ctx.accounts.flowmeter_sensor;

    flowmeter_sensor.bump = ctx.bumps.flowmeter_sensor;
    flowmeter_sensor.host = ctx.accounts.host.key();
    flowmeter_sensor.model = SensorType::Flowmeter;
    flowmeter_sensor.id = sensor_host.flowmeter_sensor_counter;
    flowmeter_sensor.status = SensorStatus::Uncollateralized;
    flowmeter_sensor.latitude = latitude;
    flowmeter_sensor.longitude = longitude;
    flowmeter_sensor.last_collateralized_at = 0;
    flowmeter_sensor.last_uncollateralized_at = 0;
    flowmeter_sensor.last_slashed_at = 0;

    sensor_host.flowmeter_sensor_counter += 1;

    Ok(())
}

#[derive(Accounts)]
pub struct RegisterFlowmeterSensor<'info> {
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
        seeds = [FLOWMETER_SENSOR_SEED.as_bytes(), sensor_host_state.key().as_ref(), &sensor_host_state.flowmeter_sensor_counter.to_le_bytes()],
        bump
    )]
    pub flowmeter_sensor: Account<'info, Sensor>,
    pub system_program: Program<'info, System>,
}
