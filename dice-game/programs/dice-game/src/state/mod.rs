use anchor_lang::prelude::*;

// Define the `Bet` struct, which represents the data structure for a bet within the program.
#[account]
pub struct Bet {
    // The public key of the player who placed the bet.
    pub player: Pubkey,
    
    // A unique seed to distinguish this bet from others.
    pub seed: u64,

    // The amount of SOL (or another token) wagered in the bet.
    pub amount: u64,

    // The dice number rolled, stored as an 8-bit unsigned integer.
    pub roll: u8,

    // The slot (block height) at which this bet will expire.
    pub slot: u64,

    // A bump value used to derive the Program Derived Address (PDA) for the bet account.
    pub bump: u8,
}

impl Bet {
    // Define the constant `LEN` to calculate the size of the `Bet` struct in bytes.
    // This is important for allocating space when creating the account on-chain.
    // The size is calculated as the sum of:
    //  - 8 bytes for account discriminator (implicitly added by Anchor)
    //  - 32 bytes for `player` (Pubkey)
    //  - 8 bytes each for `seed`, `amount`, and `slot`
    //  - 1 byte each for `roll` and `bump`
    pub const LEN: usize = 8 + 32 + 8*3 + 1*2;

    // A method to serialize the `Bet` struct into a byte vector, which is useful for
    // passing data between the client and the on-chain program.
    // This process is often called "instruction introspection."
    pub fn to_slice(&self) -> Vec<u8> {
        // Start by converting the `player` public key into bytes and storing it in a vector.
        let mut s = self.player.to_bytes().to_vec();
        
        // Append the byte representation of each field to the vector.
        s.extend_from_slice(&self.seed.to_le_bytes());   // Convert `seed` to little-endian bytes
        s.extend_from_slice(&self.amount.to_le_bytes()); // Convert `amount` to little-endian bytes
        s.extend_from_slice(&self.roll.to_le_bytes());   // Convert `roll` to little-endian bytes
        s.extend_from_slice(&self.slot.to_le_bytes());   // Convert `slot` to little-endian bytes
        s.extend_from_slice(&self.bump.to_le_bytes());   // Convert `bump` to little-endian bytes
        
        // Return the final serialized byte vector.
        s
    }
}
