use anchor_lang::prelude::*;
use anchor_spl::{
    token_2022::{Burn, TransferChecked},
    token_interface::{self, Mint, TokenAccount, TokenInterface},
};

use crate::{
    error::ErrorCode, program::DeGarden, Sensor, SensorHost, SensorStatus, SensorType, Vault, SENSOR_COLLATERAL_AMOUNT, SENSOR_HOST_SEED, TOKEN_MINT_SEED, VAULT_SEED
};

pub fn slash_collateral_handler(
    ctx: Context<SlashCollateral>,
    _host: Pubkey,
    _sensor_seed: String,
    _sensor_id: u64,
) -> Result<()> {
    let sensor = &mut ctx.accounts.sensor;

    if sensor.status != SensorStatus::Collateralized {
        return err!(ErrorCode::WrongSensorStatus);
    }

    let burn_cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Burn {
            mint: ctx.accounts.token_mint.to_account_info(),
            from: ctx.accounts.vault_token_ata.to_account_info(),
            authority: ctx.accounts.vault.to_account_info(),
        },
    );
    // burn 50%
    token_interface::burn(burn_cpi_ctx, SENSOR_COLLATERAL_AMOUNT / 2)?;

    let vault_seeds: &[&[u8]] = &[VAULT_SEED.as_bytes(), &[ctx.accounts.vault.bump]];
    let signer_seeds: &[&[&[u8]]] = &[vault_seeds];

    let transfer_checked_cpi_context = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        TransferChecked {
            from: ctx.accounts.vault_token_ata.to_account_info(),
            to: ctx.accounts.host_token_ata.to_account_info(),
            authority: ctx.accounts.vault.to_account_info(),
            mint: ctx.accounts.token_mint.to_account_info(),
        },
        signer_seeds,
    );

    // give 50% back to the host
    token_interface::transfer_checked(
        transfer_checked_cpi_context,
        SENSOR_COLLATERAL_AMOUNT / 2,
        ctx.accounts.token_mint.decimals,
    )?;

    sensor.status = SensorStatus::Slashed;

    Ok(())
}

#[derive(Accounts)]
#[instruction(_host: Pubkey, _sensor_seed: String, _sensor_id: u64)]
pub struct SlashCollateral<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        constraint = program.programdata_address()? == Some(program_data.key())
    )]
    pub program: Program<'info, DeGarden>,
    #[account(
        constraint = program_data.upgrade_authority_address == Some(authority.key())
    )]
    pub program_data: Account<'info, ProgramData>,
    #[account(
        seeds = [SENSOR_HOST_SEED.as_bytes(), _host.key().as_ref()],
        bump = sensor_host_state.bump
    )]
    pub sensor_host_state: Account<'info, SensorHost>,
    #[account(
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
        seeds = [VAULT_SEED.as_bytes()],
        bump = vault.bump
    )]
    pub vault: Account<'info, Vault>,
    #[account(
        associated_token::mint = token_mint,
        associated_token::authority = vault,
        associated_token::token_program = token_program
    )]
    pub vault_token_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        associated_token::mint = token_mint,
        associated_token::authority = _host,
        associated_token::token_program = token_program
    )]
    pub host_token_ata: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
}
