use anchor_lang::prelude::*;

#[constant]
pub const SENSOR_HOST_SEED: &str = "SENSOR_HOST";

#[constant]
pub const SENSOR_SEED: &str = "SENSOR";

#[constant]
pub const TOKEN_MINT_SEED: &str = "TOKEN_MINT";

#[constant]
pub const GLOBAL_STATE_SEED: &str = "GLOBAL_STATE";

#[constant]
pub const VAULT_SEED: &str = "VAULT";

#[constant]
pub const FEE_SEED: &str = "FEE";

#[constant]
pub const MINT_DECIMALS: u8 = 9;

#[constant]
pub const SENSOR_COLLATERAL_AMOUNT: u64 = 100 * MINT_DECIMALS as u64;

#[constant]
pub const SENSOR_DATA_REQUEST_COST: u64 = 10_000_000; // 1 / 100 token
