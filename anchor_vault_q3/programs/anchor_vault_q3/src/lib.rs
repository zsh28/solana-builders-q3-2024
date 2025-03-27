#![allow(unexpected_cfgs)]
use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};

declare_id!("2ui8YnXheP6xCtMC4p9KHhRBSHfBD9BKKm1gdLD4nBKJ");

#[program]
pub mod anchor_vault_q3 {
    use super::*;

    // Function to initialize the vault and vault state.
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        // Call the initialize method in the Initialize struct.
        ctx.accounts.initialize(&ctx.bumps)?;
        Ok(())
    }

    // Function to deposit funds into the vault.
    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)?;
        Ok(())
    }

    // Function to withdraw funds from the vault.
    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw(amount)?;
        Ok(())
    }

    // Function to close the vault and return the remaining funds to the user.
    pub fn close(ctx: Context<Close>) -> Result<()> {
        ctx.accounts.close()?;
        Ok(())
    }
}

// Struct to define the accounts needed for initializing the vault.
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    // The user who is the signer and will fund the vault with lamports.
    pub user: Signer<'info>,

    #[account(
        init,
        // The user is the payer for the account initialization.
        payer = user,
        // Derive the PDA (Program Derived Address) for the vault state using the user's public key.
        seeds = [b"state", user.key().as_ref()], 
        // Specify the bump seed for the PDA. The bump is derived from the canonical bump seed, which ranges from 255 to 0.
        bump,
        // Specify the space required for the VaultState account.
        space = VaultState::INIT_SPACE,
    )]
    // Account to store the state of the vault.
    pub vault_state: Account<'info, VaultState>,

    #[account(
        // This is the PDA for the vault account. It is not initialized with `init` because we are only creating a PDA.
        // PDAs are only initialized if they need to be rent-exempt.
        seeds = [b"vault", vault_state.key().as_ref()],
        bump,
    )]
    pub vault: SystemAccount<'info>,

    // The system program is needed to create accounts and perform other system-level operations.
    pub system_program: Program<'info, System>,
}

// Implementation block for the Initialize struct.
impl<'info> Initialize<'info> {
    // Function to initialize the vault state with the bump seeds.
    pub fn initialize(&mut self, bumps: &InitializeBumps) -> Result<()> {
        self.vault_state.vault_bump = bumps.vault;
        self.vault_state.state_bump = bumps.vault_state;
        Ok(())
    }
}

// Struct to define the accounts needed for depositing funds into the vault.
#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    // The user who is depositing funds.
    pub user: Signer<'info>,

    #[account(
        mut,
        // Derive the PDA for the vault account using the vault state.
        seeds = [b"vault", vault_state.key().as_ref()], 
        bump = vault_state.vault_bump,
    )]
    // The vault account where the funds will be deposited.
    pub vault: SystemAccount<'info>,

    #[account(
        seeds = [b"state", user.key().as_ref()],
        bump = vault_state.state_bump,
    )]
    // The state account that keeps track of the vault.
    pub vault_state: Account<'info, VaultState>,

    // The system program is required to perform the transfer.
    pub system_program: Program<'info, System>,
}

// Implementation block for the Deposit struct.
impl<'info> Deposit<'info> {
    // Function to transfer funds from the user's account to the vault.
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();

        // Define the accounts involved in the transfer.
        let cpi_accounts = Transfer {
            from: self.user.to_account_info(),
            to: self.vault.to_account_info(),
        };

        // Create a CPI (Cross-Program Invocation) context for the transfer.
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        // Perform the transfer of the specified amount.
        transfer(cpi_ctx, amount)?;

        Ok(())
    }
}

// Struct to define the accounts needed for withdrawing funds from the vault.
#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    // The user who is withdrawing funds.
    pub user: Signer<'info>,

    #[account(
        mut,
        // Derive the PDA for the vault account using the vault state.
        seeds = [b"vault", vault_state.key().as_ref()],
        bump = vault_state.vault_bump,
    )]
    // The vault account from which the funds will be withdrawn.
    pub vault: SystemAccount<'info>,

    #[account(
        seeds = [b"state", user.key().as_ref()],
        bump = vault_state.state_bump,
    )]
    // The state account that keeps track of the vault.
    pub vault_state: Account<'info, VaultState>,

    // The system program is required to perform the transfer.
    pub system_program: Program<'info, System>,
}

// Implementation block for the Withdraw struct.
impl<'info> Withdraw<'info> {
    // Function to transfer funds from the vault back to the user's account.
    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();

        // Define the accounts involved in the transfer.
        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.user.to_account_info(),
        };

        // Define the seeds required for the vault PDA.
        let seeds = &[
            b"vault",
            self.vault_state.to_account_info().key.as_ref(),
            &[self.vault_state.vault_bump],
        ];

        // Create the signer seeds using the vault's bump seed.
        let signer_seeds = &[&seeds[..]];

        // Create a CPI context for the transfer with the signer seeds.
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        // Perform the transfer of the specified amount.
        transfer(cpi_ctx, amount)?;

        Ok(())
    }
}

// Struct to define the accounts needed for closing the vault and returning the remaining funds.
#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut)]
    // The user who is closing the vault. This account must be mutable because funds will be transferred to it.
    pub user: Signer<'info>,

    #[account(
        mut,
        // Define the PDA (Program Derived Address) for the vault using the vault state's public key.
        // The vault account must be mutable because funds will be transferred from it.
        seeds = [b"vault", vault_state.key().as_ref()],
        bump = vault_state.vault_bump, // The bump seed for the vault PDA, which ensures the correct account is derived.
    )]
    // The vault account that will be closed. This account holds the lamports that will be transferred to the user.
    pub vault: SystemAccount<'info>,

    #[account(
        mut,
        // Define the PDA for the vault state using the user's public key.
        seeds = [b"state", user.key().as_ref()],
        bump = vault_state.state_bump, // The bump seed for the vault state PDA.
        close = user, // After this instruction is executed, the vault state account will be closed, and any remaining rent-exempt lamports will be transferred to the user's account.
    )]
    // The vault state account that keeps track of the vault's state. This account will be closed after the transaction.
    pub vault_state: Account<'info, VaultState>,

    // The system program is needed to perform the transfer of lamports and to close the accounts.
    pub system_program: Program<'info, System>,
}

//Close struct.
impl<'info> Close<'info> {
    // Function to close the vault and return the remaining lamports to the user.
    pub fn close(&mut self) -> Result<()> {
        // Retrieve the system program account information, which will be used for the CPI (Cross-Program Invocation).
        let cpi_program = self.system_program.to_account_info();

        // Define the accounts involved in the transfer from the vault to the user's account.
        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(), // Source of the lamports (the vault PDA).
            to: self.user.to_account_info(), // Destination of the lamports (the user's account).
        };

        // Define the seeds required to derive the vault PDA.
        let seeds = &[
            b"vault", // Prefix seed to ensure the uniqueness of the vault's PDA.
            self.vault_state.to_account_info().key.as_ref(), // The public key of the vault state is used to derive the vault PDA.
            &[self.vault_state.vault_bump], // The bump seed to correctly derive the vault PDA.
        ];

        // Create the signer seeds using the vault's bump seed. This allows the program to sign transactions on behalf of the vault PDA.
        let signer_seeds = &[&seeds[..]];

        // Create a CPI context for the transfer with the signer seeds.
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        // Transfer all remaining lamports from the vault to the user's account.
        // The amount being transferred is determined by the total lamports in the vault.
        transfer(cpi_ctx, self.vault.lamports())?;

        // After this transfer, the vault account will be empty, and the vault state account will be closed,
        // returning any remaining rent-exempt lamports to the user's account (handled automatically by the `close = user` attribute).
        Ok(())
    }
}


// Struct to define the state of the vault.
#[account]
pub struct VaultState {
    pub vault_bump: u8, // 8 bits -> 1 byte for the bump seed of the vault PDA.
    pub state_bump: u8, // 8 bits -> 1 byte for the bump seed of the vault state PDA.
}

// Implementation block for calculating the space needed for the VaultState account.
impl Space for VaultState {
    // The required space includes 8 bytes for the account discriminator (used by Anchor)
    // and 1 byte each for the vault_bump and state_bump fields.
    const INIT_SPACE: usize = 8 + 1 + 1;
}
