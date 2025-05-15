use anchor_lang::prelude::*;
use anchor_spl::{
    token_2022::{Burn, TransferChecked},
    token_interface::{self, transfer_checked, Mint, TokenAccount, TokenInterface},
};

use crate::{
    error::ErrorCode, events::PaySensorDataRequest, Sensor, SensorHost, SensorStatus,
    SENSOR_DATA_REQUEST_COST, SENSOR_HOST_SEED, TOKEN_MINT_SEED,
};

pub fn pay_sensor_data_handler(
    ctx: Context<PaySensorData>,
    _sensor_seed: String,
    _host: Pubkey,
    sensor_id: u64,
) -> Result<()> {
    let sensor = &mut ctx.accounts.sensor;

    if sensor.status != SensorStatus::Collateralized {
        return err!(ErrorCode::WrongSensorStatus);
    }

    // TODO: 3 senzora dobijaju, od 9 tokena 1. dobija 4 2. 3 3. 2

    // 10% is burnt
    let sensor_income_amount = SENSOR_DATA_REQUEST_COST * 90 / 100;
    let burn_amount = SENSOR_DATA_REQUEST_COST * 10 / 100;

    let transfer_checked_accounts = TransferChecked {
        from: ctx.accounts.payer_token_ata.to_account_info(),
        to: ctx.accounts.host_token_ata.to_account_info(),
        authority: ctx.accounts.payer.to_account_info(),
        mint: ctx.accounts.mint.to_account_info(),
    };

    let transfer_checked_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        transfer_checked_accounts,
    );

    transfer_checked(
        transfer_checked_ctx,
        sensor_income_amount,
        ctx.accounts.mint.decimals,
    )?;

    let burn_cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Burn {
            mint: ctx.accounts.mint.to_account_info(),
            from: ctx.accounts.payer_token_ata.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
        },
    );
    token_interface::burn(burn_cpi_ctx, burn_amount)?;

    emit!(PaySensorDataRequest {
        sensor_id: sensor_id,
        payer: ctx.accounts.payer.key()
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(_sensor_seed: String, _host: Pubkey, sensor_id: u64)]
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
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = _host,
        associated_token::token_program = token_program
    )]
    pub host_token_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        seeds = [TOKEN_MINT_SEED.as_bytes()],
        bump
    )]
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        seeds = [SENSOR_HOST_SEED.as_bytes(), _host.as_ref()],
        bump = sensor_host.bump
    )]
    pub sensor_host: Account<'info, SensorHost>,
    #[account(
        seeds = [_sensor_seed.as_bytes(), sensor_host.key().as_ref(), &sensor_id.to_le_bytes()],
        bump = sensor.bump
    )]
    pub sensor: Account<'info, Sensor>,
    pub token_program: Interface<'info, TokenInterface>,
}
