use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Wrong sensor status")]
    WrongSensorStatus,
    #[msg("Overflow")]
    Overflow,
}
