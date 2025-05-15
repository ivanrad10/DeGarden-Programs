use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{SensorHost, SENSOR_HOST_SEED, TOKEN_MINT_SEED};

pub fn add_host_handler(ctx: Context<AddHost>) -> Result<()> {
    let sensor_host_state = &mut ctx.accounts.sensor_host_state;

    sensor_host_state.address = ctx.accounts.host.key();
    sensor_host_state.moisture_sensor_counter = 0;
    sensor_host_state.flowmeter_sensor_counter = 0;
    sensor_host_state.bump = ctx.bumps.sensor_host_state;

    Ok(())
}

#[derive(Accounts)]
pub struct AddHost<'info> {
    #[account(mut)]
    pub host: Signer<'info>,
    #[account(
        init,
        payer = host,
        space = 8 + SensorHost::INIT_SPACE,
        seeds = [SENSOR_HOST_SEED.as_bytes(), host.key().as_ref()],
        bump
    )]
    pub sensor_host_state: Account<'info, SensorHost>,
    #[account(
        seeds = [TOKEN_MINT_SEED.as_bytes()],
        bump
    )]
    pub token_mint: InterfaceAccount<'info, Mint>,
    #[account(
        init,
        payer = host,
        associated_token::mint = token_mint,
        associated_token::authority = host,
        associated_token::token_program = token_program
    )]
    pub host_token_ata: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}
