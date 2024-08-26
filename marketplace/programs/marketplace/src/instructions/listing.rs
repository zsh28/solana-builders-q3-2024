use anchor_lang::prelude::*;
use anchor_spl::{
    token_interface::{TokenAccount, Mint, TokenInterface, TransferChecked, transfer_checked}, 
    metadata::{Metadata, MetadataAccount, MasterEditionAccount}, 
    associated_token::AssociatedToken,
};

use crate::state::{Listing, Marketplace};

#[derive(Accounts)]
pub struct List<'info> {
    #[account(mut)]
    maker: Signer<'info>,  // The user who is listing the NFT on the marketplace

    #[account(
        seeds = [b"marketplace", marketplace.name.as_str().as_bytes()],
        bump = marketplace.bump,
    )]
    marketplace: Box<Account<'info, Marketplace>>,  // The marketplace account derived from its name using PDA

    maker_mint: Box<InterfaceAccount<'info, Mint>>,  // The mint account of the NFT being listed

    collection_mint: Box<InterfaceAccount<'info, Mint>>,  // The mint account for the NFT's collection

    #[account(
        mut,
        associated_token::authority = maker,
        associated_token::mint = maker_mint,
    )]
    maker_ata: Box<InterfaceAccount<'info, TokenAccount>>,  // The associated token account (ATA) holding the NFT for the maker

    #[account(
        init_if_needed,
        payer = maker,
        associated_token::mint = maker_mint,
        associated_token::authority = listing,
    )]
    vault: Box<InterfaceAccount<'info, TokenAccount>>,  // The vault where the NFT will be stored (escrow account)

    #[account(
        init,
        payer = maker,
        space = Listing::INIT_SPACE,
        seeds = [marketplace.key().as_ref(), maker_mint.key().as_ref()],
        bump,
    )]
    listing: Box<Account<'info, Listing>>,  // The account storing details of the NFT listing

    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            maker_mint.key().as_ref(),
        ],
        seeds::program = metadata_program.key(),
        bump,
        constraint = metadata.collection.as_ref().unwrap().key.as_ref() == collection_mint.key().as_ref(),
        constraint = metadata.collection.as_ref().unwrap().verified == true,
    )]
    metadata: Box<Account<'info, MetadataAccount>>,  // Metadata account for the NFT, ensuring it's part of the correct and verified collection

    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            maker_mint.key().as_ref(),
            b"edition",
        ],
        seeds::program = metadata_program.key(),
        bump,
    )]
    master_edition: Box<Account<'info, MasterEditionAccount>>,  // Master edition account for verifying the NFT's authenticity

    metadata_program: Program<'info, Metadata>,  // Program handling the metadata accounts
    associated_token_program: Program<'info, AssociatedToken>,  // Program for associated token accounts management
    system_program: Program<'info, System>,  // System program for account creation
    token_program: Interface<'info, TokenInterface>,  // Token interface for managing token transfers
}

impl<'info> List<'info> {
    // Function to create a listing with the specified price and set the Listing state
    pub fn create_listing(&mut self, price: u64, bumps: &ListBumps) -> Result<()> {
        self.listing.set_inner(Listing {
            maker: self.maker.key(),  // Public key of the maker
            mint: self.maker_mint.key(),  // Public key of the NFT mint
            price,  // Listing price for the NFT
            bump: bumps.listing,  // Bump seed for the Listing PDA
        });

        Ok(())
    }

    // Function to deposit the NFT into the vault (escrow) account
    pub fn deposit_nft(&mut self) -> Result<()> {
        // Prepare the accounts required for the transfer
        let accounts = TransferChecked {
            from: self.maker_ata.to_account_info(),  // Source (maker's token account)
            to: self.vault.to_account_info(),  // Destination (vault account)
            authority: self.maker.to_account_info(),  // Authority (maker's public key)
            mint: self.maker_mint.to_account_info(),  // Mint account of the NFT
        };

        // Create CPI context for transferring the NFT
        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), accounts);

        // Execute the transfer of 1 NFT token
        transfer_checked(cpi_ctx, 1, self.maker_mint.decimals)
    }
}
