use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint, TokenAccount, TokenInterface}};

use crate::{Sensor, SensorHost, SensorStatus, SENSOR_HOST_SEED, SENSOR_SEED, TOKEN_MINT_SEED};

pub fn register_sensor_handler(ctx: Context<RegisterSensor>, latitude: i64, longitude: i64) -> Result<()> {
    let sensor_host = &mut ctx.accounts.sensor_host;
    let sensor = &mut ctx.accounts.sensor;
    
    sensor.id = sensor_host.sensor_counter;
    sensor.status = SensorStatus::Uncollateralized;
    sensor.latitude = latitude;
    sensor.longitude = longitude;
    sensor.last_collateralized_at = 0;
    sensor.last_uncollateralized_at = 0;
    sensor.last_slashed_at = 0;

    sensor_host.sensor_counter += 1;

    Ok(())
}

#[derive(Accounts)]
pub struct RegisterSensor<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        seeds = [SENSOR_HOST_SEED.as_bytes(), signer.key().as_ref()],
        bump = sensor_host.bump
    )]
    // TODO: sensor_host_state ,,,, singer -> host
    pub sensor_host: Account<'info, SensorHost>,
    #[account(
        init,
        payer = signer,
        space = 8 + Sensor::INIT_SPACE,
        seeds = [SENSOR_SEED.as_bytes(), sensor_host.key().as_ref(), &sensor_host.sensor_counter.to_le_bytes()],
        bump
    )]
    pub sensor: Account<'info, Sensor>,
    #[account(
        init,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = signer,
        associated_token::token_program = token_program
    )]
    pub sensor_host_token_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        seeds = [TOKEN_MINT_SEED.as_bytes()],
        bump 
    )]
    pub mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>
}