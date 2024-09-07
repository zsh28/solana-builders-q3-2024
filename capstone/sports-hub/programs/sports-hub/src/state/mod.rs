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
    pub event_id: u64,
    pub amount: u64,
    pub claimable: bool, // true if the user can claim their winnings
    pub is_won: bool,    // true if the user has won and claimed their reward
    pub bump: u8,
    pub outcome: u8,  // 0 for team_a, 1 for team_b
}

impl Bet {
    pub const LEN: usize = 8 + 32 + 8 + 1 + 1 + 1 + 1;

    pub fn to_slice(&self) -> Vec<u8> {
        let mut s = self.user.to_bytes().to_vec();   
        s.append(&mut self.event_id.to_le_bytes().to_vec());
        s.append(&mut self.amount.to_le_bytes().to_vec());
        s.push(self.claimable as u8);
        s.push(self.is_won as u8);
        s.push(self.bump);
        s.push(self.outcome);
        s
    }
}
