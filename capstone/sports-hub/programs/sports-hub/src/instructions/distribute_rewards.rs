use anchor_lang::{prelude::*, system_program::{Transfer, transfer}};
use crate::state::{Event, Bet};
use crate::errors::CustomError;

#[derive(Accounts)]
pub struct DistributeRewards<'info> {
    #[account(mut)]
    pub vault: SystemAccount<'info>,
    #[account(mut)]
    pub event: Account<'info, Event>,
    #[account(mut)]
    pub bet: Account<'info, Bet>, // The player's bet account
    #[account(mut)]
    pub player: Signer<'info>, // The player claiming their reward
    pub system_program: Program<'info, System>,
}

impl<'info> DistributeRewards<'info> {
    pub fn claim_reward(&mut self, event_id: u64) -> Result<()> {
        // Ensure the event has been resolved
        require!(self.event.resolved, CustomError::EventNotResolved);
        
        // Ensure that the bet is claimable and hasn't already been claimed
        require!(self.bet.claimable, CustomError::AlreadyClaimed);
        
        // Ensure that the event_id matches the event
        require!(self.event.event_id == event_id, CustomError::InvalidEvent);

        // Check if the player won the bet
        let player_bet_outcome = if self.event.winning_outcome.unwrap() == 0 {
            self.bet.amount == self.event.outcome_a_bets
        } else {
            self.bet.amount == self.event.outcome_b_bets
        };

        // Ensure the player's bet was on the winning outcome
        require!(player_bet_outcome, CustomError::BetLost);

        // Calculate the reward (initial bet + 50%)
        let reward = self.bet.amount
            .checked_add(self.bet.amount / 2)
            .ok_or(CustomError::RewardCalculationFailed)?;

        // Ensure the vault has sufficient funds
        require!(
            **self.vault.to_account_info().lamports.borrow() >= reward,
            CustomError::InsufficientVaultFunds
        );

        // Transfer reward from the vault to the player's account
        let transfer_ctx = CpiContext::new(
            self.system_program.to_account_info(),
            Transfer {
                from: self.vault.to_account_info(),
                to: self.player.to_account_info(),
            },
        );
        transfer(transfer_ctx, reward)?;

        // Mark the bet as claimed
        self.bet.claimable = false;
        self.bet.is_won = true;

        Ok(())
    }
}
