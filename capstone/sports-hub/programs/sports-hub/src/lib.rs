pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("GNB8U3zWyQUXGLjYCPHdrvq3r62g8cW67MLNGSsyLChS");

#[program]
pub mod sports_hub {
    use super::*;

    // Initialize the program and vault
    pub fn initialize(ctx: Context<Initialize>, amount: u64) -> Result<()> {
        ctx.accounts.init(amount)
    }

    // Function for placing a bet on an event
    pub fn place_bet(ctx: Context<PlaceBet>, event_id: u64, outcome: u8, amount: u64) -> Result<()> {
        ctx.accounts.place_bet(event_id, outcome, amount)
    }

    // Function for resolving an event
    pub fn resolve_event(ctx: Context<ResolveEvent>, event_id: u64, winning_outcome: u8) -> Result<()> {
        ctx.accounts.resolve_event(event_id, winning_outcome)
    }

    // Function for distributing rewards
    pub fn distribute_rewards(ctx: Context<DistributeRewards>) -> Result<()> {
        ctx.accounts.claim_reward(ctx.accounts.event.event_id)
    }

    
}