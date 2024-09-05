use anchor_lang::{prelude::*, system_program::{Transfer, transfer}};
use crate::state::Event;
use crate::errors::CustomError;

#[derive(Accounts)]
pub struct DistributeRewards<'info> {
    #[account(mut)]
    pub vault: SystemAccount<'info>,
    #[account(mut)]
    pub event: Account<'info, Event>,
    #[account(mut)]
    pub winner: Signer<'info>, // The winning player
    pub system_program: Program<'info, System>,
}

impl<'info> DistributeRewards<'info> {
    pub fn distribute_rewards(&mut self, event_id: u64) -> Result<()> {
        require!(self.event.resolved, CustomError::EventNotResolved);

        let total_bets = self.event.total_bets;
        let winning_bets = if self.event.winning_outcome.unwrap() == 0 {
            self.event.outcome_a_bets
        } else {
            self.event.outcome_b_bets
        };

        // Calculate reward: winners get their bet + 50% extra
        let reward = winning_bets
            .checked_add(winning_bets / 2)
            .unwrap();

        // Transfer reward from the vault to the winner's account
        let transfer_ctx = CpiContext::new(
            self.system_program.to_account_info(),
            Transfer {
                from: self.vault.to_account_info(),
                to: self.winner.to_account_info(),
            },
        );
        transfer(transfer_ctx, reward)?;

        Ok(())
    }
}
