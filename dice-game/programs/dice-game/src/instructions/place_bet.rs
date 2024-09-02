// Import necessary modules and traits from the Anchor framework
use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

// Import the Bet struct from the local crate's state module
use crate::state::Bet;

// Define the `PlaceBet` struct which specifies the accounts required
// for the `PlaceBet` instruction to place a bet.
#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct PlaceBet<'info> {
    // The player's account, which will sign the transaction and provide the funds for the bet.
    // This account is mutable, meaning the program can modify its state, such as deducting SOL.
    #[account(mut)]
    pub player: Signer<'info>,

    // The house account, representing the betting house or the central entity managing the bets.
    // This account is unchecked because the program does not need to enforce any specific rules on it.
    ///CHECK: This is safe
    pub house: UncheckedAccount<'info>,

    // The vault account, which will hold the funds for the bet.
    // The PDA (Program Derived Address) for the vault is generated using the house's public key as a seed.
    #[account(
        mut,
        seeds = [b"vault", house.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,

    // The bet account, which will store the details of the bet.
    // This account is initialized with a unique PDA, derived from the vault's key and a provided seed.
    // The player pays for the rent to create this account, and its size is defined by `Bet::LEN`.
    #[account(
        init,
        payer = player,
        space = Bet::LEN,
        seeds = [b"bet", vault.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump
    )]
    pub bet: Account<'info, Bet>,

    // The system program, required to perform system-level operations, such as transferring SOL.
    pub system_program: Program<'info, System>,
}

// Implementation block for the `PlaceBet` struct, defining the business logic.
impl<'info> PlaceBet<'info> {
    // Method to create and initialize a new bet.
    // It sets the details of the bet, such as the player's public key, seed, roll, amount, and the current slot.
    pub fn create_bet(
        &mut self,
        bumps: &PlaceBetBumps,  // Struct containing the bump seeds for account PDAs
        seed: u128,             // Unique seed to identify the bet
        roll: u8,               // The dice roll or similar outcome of the bet
        amount: u64,            // The amount of SOL wagered in the bet
    ) -> Result<()> {
        // Set the inner fields of the `bet` account with the provided data and the current slot.
        self.bet.set_inner(Bet {
            slot: Clock::get()?.slot,   // The current slot (block height) when the bet is created
            player: self.player.key(),  // Public key of the player placing the bet
            seed,                       // Unique seed to distinguish this bet
            roll,                       // The outcome of the bet (e.g., dice roll)
            amount,                     // Amount of SOL wagered
            bump: bumps.bet,            // Bump seed for the bet account's PDA
        });
        Ok(())
    }

    // Method to handle the deposit of funds from the player to the vault.
    // The amount specified will be transferred from the player's account to the vault.
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        // Define the accounts involved in the transfer: from the player's account to the vault.
        let accounts = Transfer {
            from: self.player.to_account_info(),
            to: self.vault.to_account_info(),
        };

        // Create the CPI (Cross-Program Invocation) context for the transfer using the system program.
        let ctx = CpiContext::new(self.system_program.to_account_info(), accounts);

        // Invoke the system program's `transfer` function to perform the actual SOL transfer.
        transfer(ctx, amount)
    }
}
