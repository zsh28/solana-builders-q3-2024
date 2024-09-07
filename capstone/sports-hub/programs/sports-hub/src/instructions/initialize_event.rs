use anchor_lang::prelude::*;
use crate::state::Event;

#[derive(Accounts)]
pub struct InitializeEvent<'info> {
    #[account(init, payer = payer, space = 8 + Event::LEN)]
    pub event: Account<'info, Event>, // Event account being initialized
    #[account(mut)]
    pub payer: Signer<'info>, // The payer for account initialization
    pub system_program: Program<'info, System>, // System program reference
}

