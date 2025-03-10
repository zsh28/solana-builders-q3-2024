use anchor_lang::prelude::*;
use std::str::FromStr;
use crate::state::Event;
use crate::constants::OWNER;

#[derive(Accounts)]
pub struct InitializeEvent<'info> {
    #[account(init, payer = payer, space = 8 + Event::LEN)] 
    pub event: Account<'info, Event>,
    #[account(mut, address = Pubkey::from_str(OWNER).unwrap())] // Restrict to owner only
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
