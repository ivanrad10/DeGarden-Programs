use anchor_lang::prelude::*;
use anchor_spl::{
    token_2022::TransferChecked,
    token_interface::{self, Mint, TokenAccount, TokenInterface},
};

use crate::{
    error::ErrorCode, Sensor, SensorHost, SensorStatus, Vault, SENSOR_COLLATERAL_AMOUNT,
    SENSOR_HOST_SEED, TOKEN_MINT_SEED, VAULT_SEED,
};

pub fn deposit_collateral_handler(ctx: Context<DespositCollateral>, _sensor_seed: String, _sensor_id: u64) -> Result<()> {
    let sensor = &mut ctx.accounts.sensor;

    if sensor.status != SensorStatus::Uncollateralized {
        return err!(ErrorCode::WrongSensorStatus);
    }

    let transfer_checked_cpi_context = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        TransferChecked {
            from: ctx.accounts.host_token_ata.to_account_info(),
            to: ctx.accounts.vault_token_ata.to_account_info(),
            authority: ctx.accounts.host.to_account_info(),
            mint: ctx.accounts.token_mint.to_account_info(),
        },
    );

    token_interface::transfer_checked(
        transfer_checked_cpi_context,
        SENSOR_COLLATERAL_AMOUNT,
        ctx.accounts.token_mint.decimals,
    )?;

    sensor.status = SensorStatus::Collateralized;
    sensor.last_collateralized_at = Clock::get()?.unix_timestamp;

    Ok(())
}

#[derive(Accounts)]
#[instruction(_sensor_seed: String, _sensor_id: u64)]
pub struct DespositCollateral<'info> {
    #[account(mut)]
    pub host: Signer<'info>,
    #[account(
        seeds = [SENSOR_HOST_SEED.as_bytes(), host.key().as_ref()],
        bump = sensor_host_state.bump
    )]
    pub sensor_host_state: Account<'info, SensorHost>,
    #[account(
        mut,
        seeds = [_sensor_seed.as_bytes(), sensor_host_state.key().as_ref(), &_sensor_id.to_le_bytes()],
        bump = sensor.bump
    )]
    pub sensor: Account<'info, Sensor>,
    #[account(
        seeds = [TOKEN_MINT_SEED.as_bytes()],
        bump
    )]
    pub token_mint: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = host,
        associated_token::token_program = token_program
    )]
    pub host_token_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        seeds = [VAULT_SEED.as_bytes()],
        bump = vault.bump
    )]
    pub vault: Account<'info, Vault>,
    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = vault,
        associated_token::token_program = token_program
    )]
    pub vault_token_ata: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
}
