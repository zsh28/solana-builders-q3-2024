use anchor_lang::prelude::*;
use crate::state::Event;

#[derive(Accounts)]
pub struct InitializeEvent<'info> {
    #[account(init, payer = payer, space = 8 + Event::LEN)]
    pub event: Account<'info, Event>, // Event account being initialized
    #[account(mut)]
    pub payer: Signer<'info>, // The payer for account initialization
    pub system_program: Program<'info, System>, // System program reference
}

pub fn initialize_event(
    ctx: Context<InitializeEvent>, 
    event_id: u64, 
    team_a: [u8; 32], 
    team_b: [u8; 32], 
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
    event.resolved = false;
    event.winning_outcome = None;

    Ok(())
}
