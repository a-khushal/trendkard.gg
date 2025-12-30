use anchor_lang::prelude::*;

pub mod state;
pub mod instructions;
pub mod error;
pub mod context;

pub use context::*;

use crate::instructions::{initialize_inx, create_round_inx, submit_deck_inx, resolve_round_inx, claim_reward_inx};

declare_id!("AxjdRn3AkkhJBEwAJQogRWuw6hKK8fnjQUGLqT1RWJEn");

#[program]
pub mod trendkard_gg {
    use super::*;    
    pub fn initialize(
        ctx: Context<Initialize>,
        house_fee_bps: u16,
    ) -> Result<()> {
        initialize_inx(ctx, house_fee_bps)
    }

    pub fn create_round(
        ctx: Context<CreateRound>,
        start_ts: i64,
        end_ts: i64
    ) -> Result<()> {
        create_round_inx(ctx, start_ts, end_ts)
    }

    pub fn submit_deck(
        ctx: Context<SubmitDeck>,
        kard_ids: [Pubkey; 5]
    ) -> Result<()> {
        submit_deck_inx(ctx, kard_ids)
    }

    pub fn resolve_round(
        ctx: Context<ResolveRound>,
        submission_keys: Vec<Pubkey>,
        scores: Vec<u64>,
    ) -> Result<()> {
        resolve_round_inx(ctx, submission_keys, scores)
    }

    pub fn claim_reward(
        ctx: Context<ClaimReward>,
        amount: u64,
    ) -> Result<()> {
        claim_reward_inx(ctx, amount)
    }
}
