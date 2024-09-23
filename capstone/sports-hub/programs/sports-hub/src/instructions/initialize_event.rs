use anchor_lang::prelude::*;
use crate::state::Event;

#[derive(Accounts)]
pub struct InitializeEvent<'info> {
    #[account(init, payer = payer, space = 8 + Event::LEN)] 
    pub event: Account<'info, Event>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}