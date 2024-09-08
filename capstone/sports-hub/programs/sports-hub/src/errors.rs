use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("Event has already been resolved")]
    EventAlreadyResolved,
    #[msg("Invalid outcome provided")]
    InvalidOutcome,
    #[msg("Event not resolved yet")]
    EventNotResolved,
    #[msg("Betting closed")]
    BettingClosed,
    #[msg("Already claimed")]
    AlreadyClaimed,
    #[msg("Event not started")]
    EventNotStarted,
    #[msg("Invalid bet amount")]
    InvalidBetAmount,
    #[msg("Invalid event ID")]
    InvalidEvent,
    #[msg("Insufficient vault funds")]
    InsufficientVaultFunds,
    #[msg("Reward calculation failed")]
    RewardCalculationFailed,
    #[msg("Bet lost")]
    BetLost,
    #[msg("Bet overflow")]
    BetOverflow,
}
