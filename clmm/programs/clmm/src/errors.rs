use anchor_lang::prelude::*;

#[error_code]
pub enum SrAmmError {
    #[msg("Slippage tolerance exceeded")]
    SlippageExceeded,
    #[msg("Invalid fee percentage")]
    InvalidFee,
    #[msg("Insufficient liquidity")]
    InsufficientLiquidity,
    #[msg("Operation expired")]
    Expired,
    #[msg("Math operation overflow")]
    MathOverflow,
}