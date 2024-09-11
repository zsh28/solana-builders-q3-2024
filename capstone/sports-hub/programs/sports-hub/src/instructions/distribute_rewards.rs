use anchor_lang::{prelude::*, system_program::{Transfer, transfer}};
use crate::state::{Event, Bet, PlayerStats};
use crate::errors::CustomError;

#[derive(Accounts)]
pub struct DistributeRewards<'info> {
    #[account(
        mut,
        seeds = [b"vault", house.key().as_ref()],
        bump = event.bump  // Ensure bump matches the one used during initialization
    )]
    pub vault: SystemAccount<'info>,  // Vault PDA
    #[account(mut)]
    pub event: Account<'info, Event>,  // Event account
    #[account(mut)]
    pub bet: Account<'info, Bet>,  // Bet account
    #[account(mut)]
    pub player: Signer<'info>,  // Player
    #[account(mut)]
    pub player_stats: Account<'info, PlayerStats>,  // Player stats
    pub system_program: Program<'info, System>,
    /// CHECK: This is the house account used for PDA derivation
    pub house: AccountInfo<'info>,  // House account needed to derive the vault PDA
}


impl<'info> DistributeRewards<'info> {
    pub fn claim_reward(&mut self, _event_id: u64) -> Result<()> {
        // Ensure the event has been resolved
        require!(self.event.resolved, CustomError::EventNotResolved);

        // If the event was canceled, refund the bet amount
        if self.event.winning_outcome.is_none() {
            let refund_amount = self.bet.amount;

            // Ensure the vault has enough funds
            require!(
                self.vault.lamports() >= refund_amount,
                CustomError::InsufficientVaultFunds
            );

            let seeds = &[
                b"vault",
                self.house.key.as_ref(),
                &[self.event.bump]
            ];
            let signer_seeds = &[&seeds[..]];

            let transfer_ctx = CpiContext::new_with_signer(
                self.system_program.to_account_info(),
                Transfer {
                    from: self.vault.to_account_info(),
                    to: self.player.to_account_info(),
                },
                signer_seeds
            );
            transfer(transfer_ctx, refund_amount)?;

            // Mark the bet as claimed
            self.bet.claimable = false;
            return Ok(());
        }

        // Check if the player bet on the winning outcome
        let player_bet_outcome = self.bet.outcome == self.event.winning_outcome.unwrap();
        
        // Ensure the player's bet was on the winning outcome
        require!(player_bet_outcome, CustomError::BetLost);

        // Total bet pool (outcome A, outcome B, and draw combined)
        let total_pool = self.event.outcome_a_bets + self.event.outcome_b_bets + self.event.draw_bets;

        // Calculate the total amount bet on the winning side
        let total_winning_bets = match self.event.winning_outcome.unwrap() {
            0 => self.event.outcome_a_bets,
            1 => self.event.outcome_b_bets,
            2 => self.event.draw_bets,
            _ => return Err(CustomError::InvalidOutcome.into()),
        };

        // Platform fee (e.g., 5% of the total pool)
        let platform_fee = total_pool * 5 / 100; // 5% fee

        // Remaining pool after the fee is deducted
        let total_pool_after_fee = total_pool - platform_fee;

        // Calculate player's proportional reward
        let player_reward = (self.bet.amount as u64 * total_pool_after_fee) / total_winning_bets;

        // Ensure the vault has sufficient funds for the reward
        require!(
            self.vault.lamports() >= player_reward,
            CustomError::InsufficientVaultFunds
        );

        // Transfer the reward from the vault to the player's account
        let seeds = &[
            b"vault",
            self.house.key.as_ref(),
            &[self.event.bump]
        ];
        let signer_seeds = &[&seeds[..]];

        let transfer_ctx = CpiContext::new_with_signer(
            self.system_program.to_account_info(),
            Transfer {
                from: self.vault.to_account_info(),
                to: self.player.to_account_info(),
            },
            signer_seeds
        );
        transfer(transfer_ctx, player_reward)?;

        // Mark the bet as claimed
        self.bet.claimable = false;
        self.bet.is_won = true;

        // Update player's total winnings stats
        self.player_stats.total_winnings = self.player_stats.total_winnings
            .checked_add(player_reward)
            .ok_or(CustomError::RewardCalculationFailed)?;

        Ok(())
    }
}