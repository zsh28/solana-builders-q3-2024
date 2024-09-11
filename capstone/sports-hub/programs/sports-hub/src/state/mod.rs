use anchor_lang::prelude::*;

#[account]
pub struct Event {
    pub event_id: u64,
    pub team_a: String,    // Variable length string for team A
    pub team_b: String,    // Variable length string for team B
    pub start_time: i64,
    pub total_bets: u64,
    pub outcome_a_bets: u64,
    pub outcome_b_bets: u64,
    pub draw_bets: u64,    // New field for draw bets
    pub resolved: bool,
    pub winning_outcome: Option<u8>, // Optional value (1 byte for Some/None)
    pub bump: u8,
}

impl Event {
    pub const LEN: usize = 8      // event_id
                         + 4 + 64 // team_a (max length for String + size prefix)
                         + 4 + 64 // team_b (max length for String + size prefix)
                         + 8      // start_time
                         + 8      // total_bets
                         + 8      // outcome_a_bets
                         + 8      // outcome_b_bets
                         + 8      // draw_bets
                         + 1      // resolved (bool)
                         + 1;     // winning_outcome (Option<u8>)
}

#[account]
pub struct Bet {
    pub user: Pubkey,
    pub event_id: u64,
    pub amount: u64,
    pub claimable: bool, // true if the user can claim their winnings
    pub is_won: bool,    // true if the user has won and claimed their reward
    pub bump: u8,
    pub outcome: u8,  // 0 for team_a, 1 for team_b, 2 for draw
}

impl Bet {
    pub const LEN: usize = 8      // discriminator
                         + 32     // user Pubkey
                         + 8      // event_id
                         + 8      // amount
                         + 1      // claimable (bool)
                         + 1      // is_won (bool)
                         + 1      // bump (u8)
                         + 1;     // outcome (u8)
}

#[account]
pub struct PlayerStats {
    pub total_bets: u64,
    pub total_winnings: u64,
}

impl PlayerStats {
    pub const LEN: usize = 8 + 8; // Space for total_bets and total_winnings
}
