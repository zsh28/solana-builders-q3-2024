use anchor_lang::prelude::*;
use crate::state::Event;
use crate::constants::OWNER;
use std::str::FromStr;
//use crate::errors::CustomError;

#[derive(Accounts)]
pub struct ResolveEvent<'info> {
    #[account(mut,
        address = Pubkey::from_str(OWNER).unwrap())]
    pub admin: Signer<'info>,
    #[account(mut)]
    pub event: Account<'info, Event>,
}
