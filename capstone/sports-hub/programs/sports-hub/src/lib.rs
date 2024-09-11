pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;
pub use instructions::*;
pub use state::*;
pub use errors::*;
pub use constants::*;

declare_id!("7zqecNY5KfdS8KCyNYBrtHya8SmxXyUta3hnLEvBCZxz");

#[program]
pub mod sports_hub {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, amount: u64) -> Result<()> {
        ctx.accounts.init(amount)
    }

    pub fn create_event(ctx: Context<InitializeEvent>, team_a: String, team_b: String, duration_in_seconds: i64) -> Result<()> {
        let event = &mut ctx.accounts.event;
        
        // Fetch current time from the Solana clock
        let current_time = Clock::get()?.unix_timestamp;
        
        // Set the event details, including the start time based on the current Solana time
        event.event_id = 1; // Example event ID
        event.team_a = team_a;
        event.team_b = team_b;
        event.start_time = current_time + duration_in_seconds; // Set start time to current time + duration
        event.total_bets = 0;
        event.outcome_a_bets = 0;
        event.outcome_b_bets = 0;
        event.draw_bets = 0;
        event.resolved = false;
        event.winning_outcome = None;
        
        msg!("Event initialized with start time: {}", event.start_time);
        
        Ok(())
    }

    pub fn place_bet(ctx: Context<PlaceBet>, event_id: u64, outcome: u8, amount: u64) -> Result<()> {
        let event = &mut ctx.accounts.event;
    
        // Ensure the event ID matches
        require!(event.event_id == event_id, CustomError::InvalidEvent);
        
        // Ensure the event is not already resolved
        require!(!event.resolved, CustomError::EventAlreadyResolved);
        
        // Fetch current time and ensure the bet is placed before the event start time
        let current_time = Clock::get()?.unix_timestamp;
        msg!("Current time: {}, Event start time: {}", current_time, event.start_time);
        
        require!(current_time < event.start_time, CustomError::BettingClosed);
        
        // Ensure the outcome is valid (0 for Team A, 1 for Team B, 2 for draw)
        require!(outcome == 0 || outcome == 1 || outcome == 2, CustomError::InvalidOutcome);
        
        // Ensure the bet amount is greater than 0
        require!(amount > 0, CustomError::InvalidBetAmount);
    
        // Add the bet to the respective outcome pool
        match outcome {
            0 => {
                event.outcome_a_bets = event.outcome_a_bets.checked_add(amount).ok_or(CustomError::BetOverflow)?;
            },
            1 => {
                event.outcome_b_bets = event.outcome_b_bets.checked_add(amount).ok_or(CustomError::BetOverflow)?;
            },
            2 => {
                event.draw_bets = event.draw_bets.checked_add(amount).ok_or(CustomError::BetOverflow)?;
            },
            _ => return Err(CustomError::InvalidOutcome.into()),
        }
    
        // Add the bet amount to the total bets
        event.total_bets = event.total_bets.checked_add(amount).ok_or(CustomError::BetOverflow)?;
    
        // Initialize the bet account
        let bet = &mut ctx.accounts.bet;
        bet.user = *ctx.accounts.player.key;
        bet.event_id = event_id;
        bet.amount = amount;
        bet.outcome = outcome;
        bet.claimable = false;
        bet.is_won = false;
    
        // Update player stats
        let player_stats = &mut ctx.accounts.player_stats;
        player_stats.total_bets = player_stats.total_bets.checked_add(amount).ok_or(CustomError::BetOverflow)?;
    
        // Transfer SOL to the vault
        let transfer_ctx = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            anchor_lang::system_program::Transfer {
                from: ctx.accounts.player.to_account_info(),
                to: ctx.accounts.vault.to_account_info(),
            },
        );
        anchor_lang::system_program::transfer(transfer_ctx, amount)?;
    
        Ok(())
    }
    

    pub fn resolve_event(ctx: Context<ResolveEvent>, event_id: u64, winning_outcome: Option<u8>) -> Result<()> {
        let event = &mut ctx.accounts.event;
    
        // Ensure the event ID matches
        require!(event.event_id == event_id, CustomError::InvalidEvent);
        
        // Ensure the event is not already resolved
        require!(!event.resolved, CustomError::EventAlreadyResolved);
    
        // If no winning outcome is provided, the event is considered canceled
        if winning_outcome.is_none() {
            event.resolved = true;
            event.winning_outcome = None; // Event canceled
            msg!("Event {} canceled", event_id);
            return Ok(());
        }
    
        // If there is a winning outcome (0, 1, or 2), resolve the event
        require!(
            winning_outcome == Some(0) || winning_outcome == Some(1) || winning_outcome == Some(2),
            CustomError::InvalidOutcome
        );
        event.winning_outcome = winning_outcome;
        event.resolved = true;
    
        msg!("Event {} resolved with outcome {:?}", event_id, winning_outcome);
    
        Ok(())
    }
    

    pub fn distribute_rewards(ctx: Context<DistributeRewards>) -> Result<()> {
        ctx.accounts.claim_reward(ctx.accounts.event.event_id, 0)
    }
}
