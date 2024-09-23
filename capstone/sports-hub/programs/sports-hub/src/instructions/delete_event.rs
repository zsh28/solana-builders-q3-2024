use anchor_lang::prelude::*;
use crate::state::{Event, Bet};
use crate::errors::CustomError;

#[derive(Accounts)]
pub struct DeleteEvent<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,  // Only an admin or authority should be able to delete an event.
    
    #[account(
        mut,
        close = admin, // Close the account and transfer rent to admin
        constraint = event.resolved @ CustomError::EventNotResolved,  // Ensure event is resolved
        constraint = event.all_rewards_claimed() @ CustomError::RewardsNotClaimed,  // Ensure all rewards are claimed
    )]
    pub event: Account<'info, Event>,  // Event account to delete
    
    #[account(
        mut,
        seeds = [b"bet", event.key().as_ref(), player.key().as_ref()],
        bump,
        close = player, // Close bet accounts once event is deleted
    )]
    pub bet: Account<'info, Bet>,  // Bet account associated with the event
    
    #[account(mut)]
    pub player: Signer<'info>,  // The player who placed the bet
}

impl<'info> DeleteEvent<'info> {
    pub fn delete_event(&mut self) -> Result<()> {
        // Check if the event is resolved
        require!(self.event.resolved, CustomError::EventNotResolved);

        // Check if all rewards have been claimed
        require!(self.event.all_rewards_claimed(), CustomError::RewardsNotClaimed);

        // If all checks pass, the event and associated bets will be deleted
        msg!("Event {} and all associated bets have been successfully deleted", self.event.event_id);

        Ok(())
    }
}
