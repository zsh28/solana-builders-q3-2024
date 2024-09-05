use anchor_lang::prelude::*;

//create a solana account 
//listing account
#[account]
pub struct Listing {
    pub maker: Pubkey,
    pub mint: Pubkey,
    pub price: u64,
    pub bump: u8
}

//space implementation by ourselves
impl Space for Listing{

    const INIT_SPACE: usize = 8 + 32 + 32 + 8 + 1 ;
}