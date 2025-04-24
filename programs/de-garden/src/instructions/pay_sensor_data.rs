use anchor_lang::prelude::*;
use anchor_spl::{ token_2022::TransferChecked, token_interface::{Mint, TokenAccount, TokenInterface, transfer_checked}};

use crate::{error::ErrorCode, events::PaySensorDataRequest, Sensor, SensorHost, SensorStatus, SENSOR_DATA_REQUEST_COST, SENSOR_HOST_SEED, SENSOR_SEED, TOKEN_MINT_SEED};

pub fn pay_sensor_data_handler(ctx: Context<PaySensorData>, host: Pubkey, sensor_id: u64) -> Result<()> {
    let sensor = &mut ctx.accounts.sensor;

    if sensor.status != SensorStatus::Collateralized {
        return err!(ErrorCode::WrongSensorStatus);
    }

    let transfer_checked_accounts = TransferChecked {
        from: ctx.accounts.payer_token_ata.to_account_info(),
        to: ctx.accounts.host_token_ata.to_account_info(),
        authority: ctx.accounts.payer.to_account_info(),
        mint: ctx.accounts.mint.to_account_info()
    };

    let transfer_checked_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(), 
        transfer_checked_accounts
    );

    // TODO: state for how much that sensor has earned
    // TODO: fees

    transfer_checked(
        transfer_checked_ctx,
        SENSOR_DATA_REQUEST_COST, 
        ctx.accounts.mint.decimals
    )?;

    emit!(PaySensorDataRequest {
        sensor_id: sensor_id,
        payer: ctx.accounts.payer.key()
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(host: Pubkey, sensor_id: u64)]
pub struct PaySensorData<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = payer,
        associated_token::token_program = token_program
    )]
    pub payer_token_ata: InterfaceAccount<'info, TokenAccount>,
    // TODO: add host in sensor_state
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = host,
        associated_token::token_program = token_program
    )]
    pub host_token_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        seeds = [TOKEN_MINT_SEED.as_bytes()],
        bump 
    )]
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        seeds = [SENSOR_HOST_SEED.as_bytes(), host.as_ref()],
        bump = sensor_host.bump
    )]
    pub sensor_host: Account<'info, SensorHost>,
    #[account(
        seeds = [SENSOR_SEED.as_bytes(), sensor_host.key().as_ref(), &sensor_id.to_le_bytes()],
        bump = sensor.bump
    )]
    pub sensor: Account<'info, Sensor>,
    pub token_program: Interface<'info, TokenInterface>,
}