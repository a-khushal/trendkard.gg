use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid round timestamps")]
    InvalidTimestamps,
    #[msg("Round is not active")]
    RoundNotActive,
    #[msg("Round already resolved")]
    RoundAlreadyResolved,
    #[msg("Round not resolved yet")]
    RoundNotResolved,
    #[msg("Reward already claimed")]
    AlreadyClaimed,
    #[msg("Invalid submission account")]
    InvalidSubmission,
}
