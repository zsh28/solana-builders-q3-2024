pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("2j8mbNmbxsrMSVLUWVc6zXDHxVVvEoUitAnTYqSvpPJb");

#[program]
pub mod marketplace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, fee: u16, name: String) -> Result<()> {
        ctx.accounts.init(name, fee, &ctx.bumps)
    }
}
