pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;
pub mod events;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("F9VSV1gPZQ3J9th67xYQ5yrYwePvTg3tFeFQcnPVzfKH");

#[program]
pub mod de_garden {
    use super::*;

    pub fn initialize_global_state(ctx: Context<InitializeGlobalState>, token_price_in_lamports: u64) -> Result<()> {
        instructions::initialize_global_state_handler(ctx, token_price_in_lamports)
    }

    pub fn add_host(ctx: Context<AddHost>) -> Result<()> {
        instructions::add_host_handler(ctx)
    }

    pub fn register_sensor(ctx: Context<RegisterSensor>, latitude: i64, longitude: i64) -> Result<()> {
        instructions::register_sensor_handler(ctx, latitude, longitude)
    }

    pub fn deposit_collateral(ctx: Context<DespositCollateral>, sensor_id: u64) -> Result<()> {
        instructions::deposit_collateral_handler(ctx, sensor_id)
    }

    pub fn withdraw_collateral(ctx: Context<WithdrawCollateral>, sensor_id: u64) -> Result<()> {
        instructions::withdraw_collateral_handler(ctx, sensor_id)
    }

    pub fn buy_tokens(ctx: Context<BuyTokens>, amount: u64) -> Result<()> {
        instructions::buy_tokens_handler(ctx, amount)
    }

    pub fn sell_tokens(ctx: Context<SellTokens>, amount: u64) -> Result<()> {
        instructions::sell_tokens_handler(ctx, amount)
    }

    pub fn pay_sensor_data(ctx: Context<PaySensorData>, host: Pubkey, sensor_id: u64) -> Result<()> {
        instructions::pay_sensor_data_handler(ctx, host, sensor_id)
    }

    pub fn slash_collateral(ctx: Context<AddHost>) -> Result<()> {
        Ok(())
    }

    pub fn claim_fees(ctx: Context<AddHost>) -> Result<()> {
        Ok(())
    }
}
