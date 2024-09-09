use anchor_lang::prelude::*;
use crate::state::{Event, Bet, PlayerStats};
//use crate::errors::CustomError;

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
    pub player_stats: Account<'info, PlayerStats>,
    pub system_program: Program<'info, System>,
}