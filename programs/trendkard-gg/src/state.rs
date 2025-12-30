use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct State {
    pub authority: Pubkey,
    pub house_fee_bps: u16,
    pub round_count: u64
}

#[account]
#[derive(InitSpace)]
pub struct Round {
    pub id: u64,
    pub start_ts: i64,
    pub end_ts: i64,
    pub resolved: bool,
    pub prize_pool: u64,
}

#[account]
#[derive(InitSpace)]
pub struct DeckSubmission {
    pub user: Pubkey,
    pub round: Pubkey,
    pub kard_ids: [Pubkey; 5],
    pub score: u64,
    pub claimed: bool,
}
