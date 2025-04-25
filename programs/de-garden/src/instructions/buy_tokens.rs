use anchor_lang::{prelude::*, system_program};
use anchor_spl::{
    associated_token::AssociatedToken,
    token_2022::MintTo,
    token_interface::{self, Mint, TokenAccount, TokenInterface},
};

use crate::{
    error::ErrorCode, GlobalState, Vault, GLOBAL_STATE_SEED, MINT_DECIMALS, TOKEN_MINT_SEED,
    VAULT_SEED,
};

pub fn buy_tokens_handler(ctx: Context<BuyTokens>, amount: u64) -> Result<()> {
    let global_state = &ctx.accounts.global_state;

    let token_price = global_state.token_price_in_lamports;
    let mut total_payment = token_price.checked_mul(amount).ok_or(ErrorCode::Overflow)?;
    total_payment = total_payment
        .checked_div(10_u64.checked_pow(MINT_DECIMALS as u32).unwrap())
        .ok_or(ErrorCode::Overflow)?;
    // 5% fee
    let total_payment_with_fee = total_payment * 105 / 100;

    let transfer_cpi_ctx = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        system_program::Transfer {
            from: ctx.accounts.buyer.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
        },
    );

    system_program::transfer(transfer_cpi_ctx, total_payment_with_fee)?;

    let mint_seeds: &[&[u8]] = &[TOKEN_MINT_SEED.as_bytes(), &[ctx.bumps.mint]];
    let signer_seeds: &[&[&[u8]]] = &[mint_seeds];

    let mint_to_cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.buyer_token_ata.to_account_info(),
            authority: ctx.accounts.mint.to_account_info(),
        },
        signer_seeds,
    );

    token_interface::mint_to(mint_to_cpi_ctx, amount)?;

    Ok(())
}

#[derive(Accounts)]
pub struct BuyTokens<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,
    #[account(
        seeds = [GLOBAL_STATE_SEED.as_bytes()],
        bump = global_state.bump
    )]
    pub global_state: Account<'info, GlobalState>,
    #[account(
        mut,
        seeds = [VAULT_SEED.as_bytes()],
        bump = vault.bump
    )]
    pub vault: Account<'info, Vault>,
    #[account(
        mut,
        mint::authority = mint,
        seeds = [TOKEN_MINT_SEED.as_bytes()],
        bump
    )]
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = mint,
        associated_token::authority = buyer,
        associated_token::token_program = token_program
    )]
    pub buyer_token_ata: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}
