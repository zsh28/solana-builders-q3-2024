pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;
pub use instructions::*;
pub use state::*;
pub use errors::*;
pub use constants::*;

declare_id!("GNB8U3zWyQUXGLjYCPHdrvq3r62g8cW67MLNGSsyLChS");

#[program]
pub mod sports_hub {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, amount: u64) -> Result<()> {
        ctx.accounts.init(amount)
    }

    pub fn create_event(
        ctx: Context<InitializeEvent>, 
        event_id: u64, 
        team_a: String, 
        team_b: String, 
        start_time: i64
    ) -> Result<()> {
        let event = &mut ctx.accounts.event;
        event.event_id = event_id;
        event.team_a = team_a;
        event.team_b = team_b;
        event.start_time = start_time;
        event.total_bets = 0;
        event.outcome_a_bets = 0;
        event.outcome_b_bets = 0;
        event.draw_bets = 0; // Adding draw support
        event.resolved = false;
        event.winning_outcome = None;
        Ok(())
    }

    pub fn place_bet(ctx: Context<PlaceBet>, event_id: u64, outcome: u8, amount: u64) -> Result<()> {
        ctx.accounts.place_bet(event_id, outcome, amount)
    }

    pub fn resolve_event(ctx: Context<ResolveEvent>, event_id: u64, winning_outcome: Option<u8>) -> Result<()> {
        ctx.accounts.resolve_event(event_id, winning_outcome)
    }

    pub fn distribute_rewards(ctx: Context<DistributeRewards>) -> Result<()> {
        ctx.accounts.claim_reward(ctx.accounts.event.event_id)
    }
}
