use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint, TokenAccount, TokenInterface}};

use crate::{SensorHost, SENSOR_HOST_SEED, TOKEN_MINT_SEED};

pub fn add_host_handler(ctx: Context<AddHost>) -> Result<()> {
    let sensor_host = &mut ctx.accounts.sensor_host;

    sensor_host.address = ctx.accounts.signer.key();
    sensor_host.sensor_counter = 0;
    sensor_host.bump = ctx.bumps.sensor_host;

    Ok(())
}

#[derive(Accounts)]
pub struct AddHost<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        space = 8 + SensorHost::INIT_SPACE,
        seeds = [SENSOR_HOST_SEED.as_bytes(), signer.key().as_ref()],
        bump
    )]
    pub sensor_host: Account<'info, SensorHost>,
    #[account(
        seeds = [TOKEN_MINT_SEED.as_bytes()],
        bump 
    )]
    pub token_mint: InterfaceAccount<'info, Mint>,
    #[account(
        init,
        payer = signer,
        associated_token::mint = token_mint,
        associated_token::authority = signer,
        associated_token::token_program = token_program
    )]
    pub host_token_ata: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System> 
}