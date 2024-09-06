use anchor_lang::prelude::*;
use crate::state::Event;
use crate::errors::CustomError;

#[derive(Accounts)]
pub struct ResolveEvent<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(mut)]
    pub event: Account<'info, Event>, // Loaded event account
}

impl<'info> ResolveEvent<'info> {
    pub fn resolve_event(&mut self, event_id: u64, winning_outcome: u8) -> Result<()> {
        // Ensure the event ID matches the event account
        require!(self.event.event_id == event_id, CustomError::InvalidEvent);
        
        // Ensure the event has started before resolving
        require!(Clock::get()?.unix_timestamp > self.event.start_time, CustomError::EventNotStarted);
        
        // Ensure the event has not already been resolved
        require!(!self.event.resolved, CustomError::EventAlreadyResolved);
        
        // Validate the winning outcome (0 for team_a, 1 for team_b)
        require!(winning_outcome == 0 || winning_outcome == 1, CustomError::InvalidOutcome);

        // Set the winning outcome and mark the event as resolved
        self.event.winning_outcome = Some(winning_outcome);
        self.event.resolved = true;

        Ok(())
    }
}
