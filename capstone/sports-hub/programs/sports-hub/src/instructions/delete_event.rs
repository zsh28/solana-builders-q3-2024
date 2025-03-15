use anchor_lang::prelude::*;
use crate::state::{Event, Bet};
use crate::errors::CustomError;
use std::str::FromStr;
use crate::constants::OWNER;

#[derive(Accounts)]
pub struct DeleteEvent<'info> {
    #[account(mut, address = Pubkey::from_str(OWNER).unwrap())]
    pub admin: Signer<'info>,

    #[account(
        mut,
        close = admin,
        constraint = event.all_rewards_claimed() @ CustomError::RewardsNotClaimed
    )]
    pub event: Account<'info, Event>,

    #[account(
        mut,
        seeds = [b"bet", event.key().as_ref(), player.key().as_ref()],
        bump,
        close = admin
    )]
    pub bet: Option<Account<'info, Bet>>,  // Ensure mutability

    #[account(mut)]
    pub player: Signer<'info>,
}

impl<'info> DeleteEvent<'info> {
    pub fn delete_event(&mut self) -> Result<()> {
        // Ensure that all rewards have been claimed.
        require!(self.event.all_rewards_claimed(), CustomError::RewardsNotClaimed);

        // Manually close the optional bet account if it exists.
        if let Some(bet) = self.bet.as_mut() {
            let player_info = self.player.to_account_info();
            let bet_info = bet.to_account_info();
            
            // Transfer all lamports from the bet account to the player.
            **player_info.try_borrow_mut_lamports()? += **bet_info.try_borrow_lamports()?;
            **bet_info.try_borrow_mut_lamports()? = 0;
        }

        msg!(
            "Event {} and all associated bets have been successfully deleted", 
            self.event.event_id
        );
        Ok(())
    }
}
