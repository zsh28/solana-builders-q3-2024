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
    pub fn resolve_event(&mut self, event_id: u64, winning_outcome: u8) -> Result<()> {
        require!(!self.event.resolved, CustomError::EventAlreadyResolved);
        require!(winning_outcome == 0 || winning_outcome == 1, CustomError::InvalidOutcome);

        // Set the winning outcome
        self.event.winning_outcome = Some(winning_outcome);
        self.event.resolved = true;

        Ok(())
    }
}
