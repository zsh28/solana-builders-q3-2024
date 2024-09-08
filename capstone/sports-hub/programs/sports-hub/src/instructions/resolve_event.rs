use anchor_lang::prelude::*;
use crate::state::Event;
use crate::errors::CustomError;

#[derive(Accounts)]
pub struct ResolveEvent<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(mut)]
    pub event: Account<'info, Event>,
}

impl<'info> ResolveEvent<'info> {
    pub fn resolve_event(&mut self, event_id: u64, winning_outcome: Option<u8>) -> Result<()> {
        require!(self.event.event_id == event_id, CustomError::InvalidEvent);
        require!(Clock::get()?.unix_timestamp > self.event.start_time, CustomError::EventNotStarted);
        require!(!self.event.resolved, CustomError::EventAlreadyResolved);

        // If winning_outcome is `None`, the event is considered canceled
        if winning_outcome.is_none() {
            self.event.resolved = true;
            self.event.winning_outcome = None;  // Event canceled
            return Ok(());
        }

        // If there's a valid outcome (0, 1, or 2), resolve the event
        require!(winning_outcome == Some(0) || winning_outcome == Some(1) || winning_outcome == Some(2), CustomError::InvalidOutcome);
        self.event.winning_outcome = winning_outcome;
        self.event.resolved = true;

        Ok(())
    }
}
