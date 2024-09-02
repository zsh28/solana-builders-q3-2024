use anchor_lang::{prelude::*, system_program::{Transfer, transfer}};

// Struct to define the accounts required for the `Initialize` instruction.
#[derive(Accounts)]
pub struct Initialize<'info> {
    // The house account, which will sign the transaction and provide the funds.
    // The `mut` keyword indicates that this account will be mutable, allowing
    // the program to modify its state (e.g., deduct SOL for the transfer).
    #[account(mut)]
    pub house: Signer<'info>,

    // The vault account where the funds from the house account will be transferred.
    // The `seeds` attribute is used to generate a Program Derived Address (PDA) for the vault.
    // It combines a constant prefix `b"vault"` with the house's public key to derive the address.
    // The `bump` ensures the PDA is valid.
    #[account(
        mut,
        seeds = [b"vault", house.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,

    // The system program is required to perform system-level operations,
    // such as transferring SOL between accounts.
    pub system_program: Program<'info, System>,
}

// Implementation block for the Initialize struct, defining the business logic.
impl<'info> Initialize<'info>{
    // Method to initialize the transfer of SOL from the house to the vault.
    // Takes the amount to be transferred as an argument.
    pub fn init(&mut self, amount: u64) -> Result<()>{
        // Create a Transfer struct to specify the accounts involved in the transfer.
        // The `from` account is the house, and the `to` account is the vault.
        let account: Transfer = Transfer{
            from: self.house.to_account_info(),
            to: self.vault.to_account_info(),
        };

        // Create a context (`ctx`) for the Cross-Program Invocation (CPI) using the 
        // system program and the Transfer struct.
        let ctx = CpiContext::new(self.system_program.to_account_info(), account);

        // Invoke the `transfer` function from the system program, which performs
        // the actual transfer of SOL from the house to the vault.
        // The amount of SOL to be transferred is specified by the `amount` argument.
        transfer(ctx, amount)
    }
}
