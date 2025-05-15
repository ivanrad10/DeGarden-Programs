use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{
    program::DeGarden, GlobalState, Vault, GLOBAL_STATE_SEED, MINT_DECIMALS, TOKEN_MINT_SEED,
    VAULT_SEED,
};

pub fn initialize_global_state_handler(
    ctx: Context<InitializeGlobalState>,
    token_price_in_lamports: u64,
) -> Result<()> {
    let global_state = &mut ctx.accounts.global_state;
    let vault = &mut ctx.accounts.vault;

    global_state.token_price_in_lamports = token_price_in_lamports;
    global_state.bump = ctx.bumps.global_state;

    vault.bump = ctx.bumps.vault;

    Ok(())
}

#[derive(Accounts)]
pub struct InitializeGlobalState<'info> {
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
        init,
        payer = authority,
        space = 8 + GlobalState::INIT_SPACE,
        seeds = [GLOBAL_STATE_SEED.as_bytes()],
        bump
    )]
    pub global_state: Account<'info, GlobalState>,
    #[account(
        init,
        payer = authority,
        mint::decimals = MINT_DECIMALS,
        mint::authority = token_mint,
        seeds = [TOKEN_MINT_SEED.as_bytes()],
        bump
    )]
    pub token_mint: InterfaceAccount<'info, Mint>,
    #[account(
        init,
        payer = authority,
        space = 8 + Vault::INIT_SPACE,
        seeds = [VAULT_SEED.as_bytes()],
        bump
    )]
    pub vault: Account<'info, Vault>,
    #[account(
        init,
        payer = authority,
        associated_token::mint = token_mint,
        associated_token::authority = vault,
        associated_token::token_program = token_program
    )]
    pub vault_token_ata: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}
