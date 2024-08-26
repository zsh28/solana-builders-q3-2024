use anchor_lang::prelude::*;

//list a nft on marketplace
#[account]
pub struct Listing {
    pub maker: Pubkey,
    pub mint: Pubkey,
    pub price: u64,
    pub bump: u8,
}

//impl space for listing 
// maker 32 bytes
// mint 32 bytes
// price 8 bytes
// bump 1 byte
impl Space for Listing {
    const INIT_SPACE: usize = 8+32*2+8+1; 
}