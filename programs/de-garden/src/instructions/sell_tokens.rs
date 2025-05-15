use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_2022::Burn,
    token_interface::{self, Mint, TokenAccount, TokenInterface},
};

use crate::{
    error::ErrorCode, GlobalState, Vault, GLOBAL_STATE_SEED, MINT_DECIMALS, TOKEN_MINT_SEED,
    VAULT_SEED,
};

pub fn sell_tokens_handler(ctx: Context<SellTokens>, amount: u64) -> Result<()> {
    let global_state = &ctx.accounts.global_state;

    let token_price = global_state.token_price_in_lamports;

    let mut total_payment = token_price.checked_mul(amount).ok_or(ErrorCode::Overflow)?;
    total_payment = total_payment
        .checked_div(10_u64.checked_pow(MINT_DECIMALS as u32).unwrap())
        .ok_or(ErrorCode::Overflow)?;
    let total_payment_with_fee = total_payment * 95 / 100;

    let burn_cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Burn {
            mint: ctx.accounts.mint.to_account_info(),
            from: ctx.accounts.seller_token_ata.to_account_info(),
            authority: ctx.accounts.seller.to_account_info(),
        },
    );
    token_interface::burn(burn_cpi_ctx, amount)?;

    **ctx
        .accounts
        .vault
        .to_account_info()
        .try_borrow_mut_lamports()? -= total_payment_with_fee;
    **ctx
        .accounts
        .seller
        .to_account_info()
        .try_borrow_mut_lamports()? += total_payment_with_fee;

    Ok(())
}

#[derive(Accounts)]
pub struct SellTokens<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,
    #[account(
        seeds = [GLOBAL_STATE_SEED.as_bytes()],
        bump = global_state.bump
    )]
    pub global_state: Account<'info, GlobalState>,
    #[account(
        mut,
        mint::authority = mint,
        seeds = [TOKEN_MINT_SEED.as_bytes()],
        bump
    )]
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        seeds = [VAULT_SEED.as_bytes()],
        bump = vault.bump
    )]
    pub vault: Account<'info, Vault>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = seller,
        associated_token::token_program = token_program
    )]
    pub seller_token_ata: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
