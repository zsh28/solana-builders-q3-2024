use anchor_lang::prelude::*;
use crate::state::Event;
//use crate::errors::CustomError;

#[derive(Accounts)]
pub struct ResolveEvent<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(mut)]
    pub event: Account<'info, Event>,
}
