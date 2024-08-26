use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};
use anchor_spl::{
    associated_token::AssociatedToken, 
    metadata::Metadata, 
    token_interface::{Mint, TokenAccount, TokenInterface, TransferChecked, transfer_checked, CloseAccount, close_account},
};

use crate::state::{Listing, Marketplace};

#[derive(Accounts)]
pub struct Purchase<'info> {
    #[account(mut)]
    taker: Signer<'info>,  // The buyer who is purchasing the NFT

    maker: SystemAccount<'info>,  // The seller who listed the NFT

    #[account(
        seeds = [b"marketplace", marketplace.name.as_str().as_bytes()],
        bump = marketplace.bump,
    )]
    marketplace: Box<Account<'info, Marketplace>>,  // The marketplace where the NFT is listed

    maker_mint: Box<InterfaceAccount<'info, Mint>>,  // The mint account of the NFT being purchased

    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = maker_mint,
        associated_token::authority = taker,
        associated_token::token_program = token_program,
    )]
    taker_ata: Box<InterfaceAccount<'info, TokenAccount>>,  // The buyer's associated token account for receiving the NFT

    #[account(
        mut,
        close = maker,  // The listing account will be closed after the purchase, and the remaining funds will go to the maker
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

    #[account(
        seeds = [b"treasury", marketplace.key().as_ref()],
        bump = marketplace.treasury_bump,
    )]
    treasury: SystemAccount<'info>,  // The treasury account where the marketplace fees will be sent

    pub system_program: Program<'info, System>,  // System program required for fund transfers
    pub token_program: Interface<'info, TokenInterface>,  // Token program interface for managing token transfers
    pub metadata_program: Program<'info, Metadata>,  // Program for handling NFT metadata
    associated_token_program: Program<'info, AssociatedToken>,  // Program for associated token accounts
}

impl<'info> Purchase<'info> {
    pub fn purchase(&self) -> Result<()> {    
        // Move the NFT from the vault to the buyer's (taker's) associated token account
        let seeds = &[
            &self.marketplace.key().to_bytes()[..],  // Marketplace public key
            &self.maker_mint.key().to_bytes()[..],  // NFT mint public key
            &[self.listing.bump],  // Bump seed for the listing PDA
        ];

        let signer_seeds = &[&seeds[..]];  // Wrapping seeds in the signer_seeds array

        let accounts = TransferChecked {
            from: self.vault.to_account_info(),  // Source account (vault holding the NFT)
            mint: self.maker_mint.to_account_info(),  // Mint account of the NFT
            to: self.taker_ata.to_account_info(),  // Destination account (buyer's associated token account)
            authority: self.listing.to_account_info(),  // Authority (listing account)
        };

        let cpi_ctx = CpiContext::new_with_signer(self.token_program.to_account_info(), accounts, signer_seeds);

        transfer_checked(cpi_ctx, 1, self.maker_mint.decimals)?;

        // Send the marketplace fee to the treasury
        let fee_amount = ((self.listing.price as u128 * self.marketplace.fee as u128) / 10000) as u64;

        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer { 
            from: self.taker.to_account_info(),  // Source of the fee (buyer's account)
            to: self.treasury.to_account_info(),  // Destination account (treasury)
         };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_ctx, fee_amount)?;

        // Pay the remaining amount to the maker (seller)
        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer { 
            from: self.taker.to_account_info(),  // Source of the payment (buyer's account)
            to: self.maker.to_account_info(),  // Destination account (seller's account)
         };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_ctx, self.listing.price - fee_amount)?;

        // Close the vault account and return the remaining rent to the maker
        let accounts = CloseAccount {
            account: self.vault.to_account_info(),  // The vault account holding the NFT
            destination: self.maker.to_account_info(),  // The maker (seller) will receive the remaining rent
            authority: self.listing.to_account_info(),  // The authority to close the vault is the listing account
        };

        let ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            accounts,
            signer_seeds,
        );

        close_account(ctx)?;

        Ok(())
    }
}
