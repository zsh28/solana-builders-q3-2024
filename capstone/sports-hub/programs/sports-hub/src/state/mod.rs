use anchor_lang::prelude::*;

#[account]
pub struct Event {
    pub team_a: String,
    pub team_b: String,
    pub start_time: i64,
    pub total_bets: u64,
    pub outcome_a_bets: u64,
    pub outcome_b_bets: u64,
    pub resolved: bool,
    pub winning_outcome: Option<u8>, // 0 for team_a win, 1 for team_b win
}

impl Event {
    pub const LEN: usize = 8 + (32 * 2) + 8 + 8 + 8 + 1 + 1;
}
