use anchor_lang::{prelude::*, system_program::{Transfer, transfer}};
use crate::state::Event;
use crate::errors::CustomError;

#[derive(Accounts)]
pub struct PlaceBet<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(mut)]
    pub vault: SystemAccount<'info>,
    #[account(mut)]
    pub event: Account<'info, Event>, // No has_one constraint
    pub system_program: Program<'info, System>,
}

impl<'info> PlaceBet<'info> {
    pub fn place_bet(&mut self, event_id: u64, outcome: u8, amount: u64) -> Result<()> {
        // Ensure the event ID matches
        require!(self.event.event_id == event_id, CustomError::InvalidEvent);
        require!(!self.event.resolved, CustomError::EventAlreadyResolved);
        require!(Clock::get()?.unix_timestamp < self.event.start_time, CustomError::BettingClosed);
        require!(outcome == 0 || outcome == 1, CustomError::InvalidOutcome);
        require!(amount > 0, CustomError::InvalidBetAmount);

        // Add the bet amount to the respective outcome pool
        if outcome == 0 {
            self.event.outcome_a_bets = self.event.outcome_a_bets.checked_add(amount).unwrap();
        } else {
            self.event.outcome_b_bets = self.event.outcome_b_bets.checked_add(amount).unwrap();
        }

        self.event.total_bets = self.event.total_bets.checked_add(amount).unwrap();

        // Transfer SOL from the player to the vault
        let transfer_ctx = CpiContext::new(
            self.system_program.to_account_info(),
            Transfer {
                from: self.player.to_account_info(),
                to: self.vault.to_account_info(),
            },
        );
        transfer(transfer_ctx, amount)?;

        Ok(())
    }
}
