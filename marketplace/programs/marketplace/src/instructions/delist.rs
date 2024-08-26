use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface, TransferChecked, transfer_checked, CloseAccount, close_account};
use crate::state::{Listing, Marketplace};

#[derive(Accounts)]
pub struct Delist<'info> {
    #[account(mut)]
    maker: Signer<'info>,  // The user who originally listed the NFT and now wants to delist it

    #[account(
        seeds = [b"marketplace", marketplace.name.as_str().as_bytes()],
        bump = marketplace.bump,
    )]
    marketplace: Box<Account<'info, Marketplace>>,  // The marketplace account where the NFT was listed

    maker_mint: Box<InterfaceAccount<'info, Mint>>,  // The mint account for the NFT being delisted

    #[account(
        mut,
        associated_token::authority = maker,
        associated_token::mint = maker_mint,
    )]
    maker_ata: Box<InterfaceAccount<'info, TokenAccount>>,  // The maker's associated token account (ATA) for holding the NFT

    #[account(
        mut,
        close = maker,  // The listing account will be closed and funds will be sent to the maker
        seeds = [marketplace.key().as_ref(), maker_mint.key().as_ref()],
        bump = listing.bump,
    )]
    listing: Box<Account<'info, Listing>>,  // The account storing the details of the NFT listing

    #[account(
        mut,
        associated_token::mint = maker_mint,
        associated_token::authority = listing,
    )]
    vault: Box<InterfaceAccount<'info, TokenAccount>>,  // The vault account holding the NFT during the listing period

    token_program: Interface<'info, TokenInterface>,  // Interface to the token program for managing token transfers
    system_program: Program<'info, System>,  // System program required for account management
}

impl<'info> Delist<'info> {
    // Function to withdraw the NFT from the vault and return it to the maker's associated token account
    pub fn withdraw_nft(&mut self) -> Result<()> {
        // Seeds needed for signing the transfer, derived from the marketplace and mint keys
        let seeds = &[
            &self.marketplace.key().to_bytes()[..],  // Marketplace public key
            &self.maker_mint.key().to_bytes()[..],  // Maker mint public key
            &[self.listing.bump],  // Bump seed for the listing PDA
        ];
        let signer_seeds = &[&seeds[..]];  // Wrapping seeds in the signer_seeds array

        // Prepare the accounts required for the transfer
        let accounts = TransferChecked {
            from: self.vault.to_account_info(),  // Source account (vault holding the NFT)
            to: self.maker_ata.to_account_info(),  // Destination account (maker's associated token account)
            authority: self.listing.to_account_info(),  // Authority (listing account)
            mint: self.maker_mint.to_account_info(),  // Mint account of the NFT
        };

        // Create a CPI context with signer seeds to authorize the transfer
        let cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            accounts,
            signer_seeds,
        );

        // Execute the transfer of 1 NFT token back to the maker
        transfer_checked(cpi_ctx, 1, self.maker_mint.decimals)?;

        // Close the vault account and transfer the remaining rent to the maker
        let accounts = CloseAccount {
            account: self.vault.to_account_info(),
            destination: self.maker.to_account_info(),
            authority: self.listing.to_account_info(),
        };

        let close_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            accounts,
            signer_seeds,
        );

        close_account(close_ctx)?;

        Ok(())
    }
}
