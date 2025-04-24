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
pub const MINT_DECIMALS: u64 = 1_000_000;

#[constant]
pub const SENSOR_COLLATERAL_AMOUNT: u64 = 100 * MINT_DECIMALS;
