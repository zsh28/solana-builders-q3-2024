use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};

// Importing the Marketplace state and MarketplaceError enum from your project's modules
use crate::state::Marketplace;
use crate::error::MarketplaceError;

#[derive(Accounts)]
#[instruction(name: String)]  // Instruction data passed to the Initialize context, in this case, a name of type String
pub struct Initialize<'info> {
    #[account (mut)]
    admin: Signer<'info>,  // The admin who is initializing the marketplace, marked as mutable because the balance might change

    #[account(
        init,  // Initialize the marketplace account
        space = Marketplace::INIT_SPACE,  // Allocate space for the Marketplace account
        payer = admin,  // The admin will pay for the transaction
        seeds = [b"marketplace", name.as_str().as_bytes()],  // PDA seeds to derive the marketplace account
        bump,  // PDA bump for security
    )]
    marketplace: Account<'info, Marketplace>,  // The account that will store the Marketplace state

    #[account(
        init,  // Initialize the rewards mint account
        seeds = [b"rewards", marketplace.key().as_ref()],  // PDA seeds to derive the rewards mint account
        payer = admin,  // The admin will pay for this transaction as well
        bump,  // PDA bump for security
        mint::decimals = 6,  // Set the decimal precision for the rewards mint
        mint::authority = marketplace,  // The marketplace account will be the mint authority
    )]
    rewards_mint: InterfaceAccount<'info, Mint>,  // The mint account for the rewards tokens

    #[account(
        seeds = [b"treasury", marketplace.key().as_ref()],  // PDA seeds to derive the treasury account
        bump,  // PDA bump for security
    )]
    treasury: SystemAccount<'info>,  // The treasury account, managed by the system program

    system_program: Program<'info, System>,  // Reference to the system program, required for initializing accounts
    token_program: Interface<'info, TokenInterface>,  // Reference to the token program interface for handling SPL tokens
}

// Implementation block for the Initialize struct
impl<'info> Initialize<'info> {
    // Initialization function for the marketplace
    pub fn init(&mut self, name: String, fee: u16, bumps: &InitializeBumps) -> Result<()> {
        // Check that the name length is valid (between 1 and 32 characters)
        require!(name.len() > 0 && name.len() < 33, MarketplaceError::NameToLong);

        // Set the inner state of the marketplace account with the provided values
        self.marketplace.set_inner(Marketplace {
            admin: self.admin.key(),  // Set the admin public key
            fee,  // Set the marketplace fee    
            name,  // Set the marketplace name
            bump: bumps.marketplace,  // Set the bump for the marketplace PDA
            treasury_bump: bumps.treasury,  // Set the bump for the treasury PDA
            rewards_bump: bumps.rewards_mint,  // Set the bump for the rewards mint PDA
        });

        // Return success
        Ok(())
    }
}
