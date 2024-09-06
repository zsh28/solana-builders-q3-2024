use anchor_lang::prelude::*;

#[account]
pub struct Event {
    pub event_id: u64,
    pub team_a: [u8; 32], // Fixed-size array for team names (up to 32 bytes)
    pub team_b: [u8; 32], // Same as above
    pub start_time: i64,  // Users can bet until the match/event starts
    pub total_bets: u64,
    pub outcome_a_bets: u64,
    pub outcome_b_bets: u64,
    pub resolved: bool,
    pub winning_outcome: Option<u8>, // 0 for team_a win, 1 for team_b win
}

impl Event {
    pub const LEN: usize = 8 + 32 + 32 + 8 + 8 + 8 + 8 + 1 + 1;
}


#[account]
pub struct Bet {
    pub user: Pubkey,
    pub event: Pubkey,
    pub amount: u64,
    pub claimable: bool, // true if the user can claim their winnings
    pub is_won: bool,    // true if the user has won and claimed their reward
}

impl Bet {
    pub const LEN: usize = 8 + 32 + 32 + 8 + 1 + 1;
}
