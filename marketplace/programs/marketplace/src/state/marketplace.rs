use anchor_lang::prelude::*;

// Account for marketplace (state)
#[account]
pub struct Marketplace {
    // Initializer (admin public key)
    pub admin: Pubkey,       // 32 bytes
    // Marketplace fee
    pub fee: u16,            // 2 bytes
    // Bump seed for PDA (Program Derived Address)
    pub bump: u8,            // 1 byte
    // Bump seed for rewards PDA
    pub rewards_bump: u8,    // 1 byte
    // Bump seed for treasury PDA
    pub treasury_bump: u8,   // 1 byte
    // Marketplace name
    pub name: String,        // 4 bytes (for length) + 32 bytes (for string)
}

// Implement space calculation for marketplace
// 8 bytes for account discriminator
// 32 bytes for admin (Pubkey)
// 2 bytes for fee (u16)
// 1 byte for bump (u8)
// 1 byte for rewards_bump (u8)
// 1 byte for treasury_bump (u8)
// 4 bytes for the length of the name (String prefix)
// 32 bytes for the name (assuming a maximum of 32 characters)
impl Space for Marketplace {
    const INIT_SPACE: usize = 8 + 32 + 2 + 1*3 + (32 + 4); 
}