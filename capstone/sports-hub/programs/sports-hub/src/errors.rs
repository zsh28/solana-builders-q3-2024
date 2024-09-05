use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("Event has already been resolved")]
    EventAlreadyResolved,
    #[msg("Invalid outcome provided")]
    InvalidOutcome,
    #[msg("Event not resolved yet")]
    EventNotResolved,
}
