use anchor_lang::prelude::*;

#[error_code]
pub enum MarketplaceError {
    #[msg("Name must be less than or equal to 32 charcters")]
    NameToLong,
}
