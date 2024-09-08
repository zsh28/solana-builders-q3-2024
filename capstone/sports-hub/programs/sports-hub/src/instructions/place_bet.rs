use anchor_lang::{prelude::*, system_program::{Transfer, transfer}};
use crate::state::{Event, Bet, PlayerStats};
use crate::errors::CustomError;

#[derive(Accounts)]
pub struct PlaceBet<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(mut)]
    pub vault: SystemAccount<'info>,
    #[account(mut)]
    pub event: Account<'info, Event>,
    #[account(
        init,
        payer = player,
        space = 8 + Bet::LEN,
        seeds = [b"bet", event.key().as_ref(), player.key().as_ref()],
        bump
    )]
    pub bet: Account<'info, Bet>,
    #[account(
        init_if_needed,
        payer = player,
        space = 8 + PlayerStats::LEN,
        seeds = [b"stats", player.key().as_ref()],
        bump
    )]
    pub player_stats: Account<'info, PlayerStats>, // Track player's total bets and winnings
    pub system_program: Program<'info, System>,
}

impl<'info> PlaceBet<'info> {
    pub fn place_bet(&mut self, event_id: u64, outcome: u8, amount: u64) -> Result<()> {
        // Ensure the event ID matches
        require!(self.event.event_id == event_id, CustomError::InvalidEvent);
        
        // Ensure the event is not already resolved
        require!(!self.event.resolved, CustomError::EventAlreadyResolved);
        
        // Ensure that the bet is placed before the event start time
        let current_time = Clock::get()?.unix_timestamp;
        require!(current_time < self.event.start_time, CustomError::BettingClosed);
        
        // Ensure the outcome is valid (0 for team A, 1 for team B, 2 for draw)
        require!(outcome == 0 || outcome == 1 || outcome == 2, CustomError::InvalidOutcome);
        
        // Ensure the bet amount is greater than 0
        require!(amount > 0, CustomError::InvalidBetAmount);

        // Add the bet amount to the respective outcome pool
        match outcome {
            0 => {
                self.event.outcome_a_bets = self.event.outcome_a_bets
                    .checked_add(amount)
                    .ok_or(CustomError::BetOverflow)?;
            },
            1 => {
                self.event.outcome_b_bets = self.event.outcome_b_bets
                    .checked_add(amount)
                    .ok_or(CustomError::BetOverflow)?;
            },
            2 => {
                self.event.draw_bets = self.event.draw_bets
                    .checked_add(amount)
                    .ok_or(CustomError::BetOverflow)?;
            },
            _ => return Err(CustomError::InvalidOutcome.into()),
        }

        // Add the bet amount to the total bets
        self.event.total_bets = self.event.total_bets
            .checked_add(amount)
            .ok_or(CustomError::BetOverflow)?;

        // Initialize the bet account
        self.bet.user = *self.player.key;
        self.bet.event_id = event_id;
        self.bet.amount = amount;
        self.bet.outcome = outcome;
        self.bet.claimable = false;
        self.bet.is_won = false;

        // Update player's total bet stats
        self.player_stats.total_bets = self.player_stats.total_bets
            .checked_add(amount)
            .ok_or(CustomError::BetOverflow)?;

        // Transfer SOL from the player's account to the vault
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
