pub mod constants;
pub mod error;
pub mod events;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("H3B2URUN8BMxZvxx6yit2n8sRRWGo8bBdew4yDosurjG");

#[program]
pub mod de_garden {
    use super::*;

    pub fn initialize_global_state(
        ctx: Context<InitializeGlobalState>,
        token_price_in_lamports: u64,
    ) -> Result<()> {
        instructions::initialize_global_state_handler(ctx, token_price_in_lamports)
    }

    pub fn add_host(ctx: Context<AddHost>) -> Result<()> {
        instructions::add_host_handler(ctx)
    }

    pub fn register_moisture_sensor(
        ctx: Context<RegisterMoistureSensor>,
        latitude: i64,
        longitude: i64,
    ) -> Result<()> {
        instructions::register_moisture_sensor_handler(ctx, latitude, longitude)
    }

    pub fn register_flowmeter_sensor(
        ctx: Context<RegisterFlowmeterSensor>,
        latitude: i64,
        longitude: i64,
    ) -> Result<()> {
        instructions::register_flowmeter_sensor_handler(ctx, latitude, longitude)
    }

    pub fn deposit_collateral(ctx: Context<DespositCollateral>, sensor_seed: String, sensor_id: u64) -> Result<()> {
        instructions::deposit_collateral_handler(ctx, sensor_seed, sensor_id)
    }

    pub fn withdraw_collateral(ctx: Context<WithdrawCollateral>, sensor_seed: String, sensor_id: u64) -> Result<()> {
        instructions::withdraw_collateral_handler(ctx, sensor_seed, sensor_id)
    }

    pub fn buy_tokens(ctx: Context<BuyTokens>, amount: u64) -> Result<()> {
        instructions::buy_tokens_handler(ctx, amount)
    }

    pub fn sell_tokens(ctx: Context<SellTokens>, amount: u64) -> Result<()> {
        instructions::sell_tokens_handler(ctx, amount)
    }

    pub fn pay_sensor_data(
        ctx: Context<PaySensorData>,
        _sensor_seed: String,
        host: Pubkey,
        sensor_id: u64,
    ) -> Result<()> {
        instructions::pay_sensor_data_handler(ctx, _sensor_seed, host, sensor_id)
    }

    pub fn slash_collateral(
        ctx: Context<SlashCollateral>,
        host: Pubkey,
        sensor_seed: String,
        sensor_id: u64,
    ) -> Result<()> {
        instructions::slash_collateral_handler(ctx, host, sensor_seed, sensor_id)
    }
}
