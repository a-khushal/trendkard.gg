use anchor_lang::prelude::*;
use crate::{context::{ClaimReward, CreateRound, Initialize, ResolveRound, SubmitDeck}, error::ErrorCode, state::DeckSubmission};

pub fn initialize_inx(
    ctx: Context<Initialize>,
    house_fee_bps: u16,
) -> Result<()> {
    let state = &mut ctx.accounts.state;
    state.authority = ctx.accounts.authority.key();
    state.house_fee_bps = house_fee_bps;
    state.round_count = 0;
    Ok(())
}

pub fn create_round_inx(
    ctx: Context<CreateRound>,
    start_ts: i64,
    end_ts: i64,
) -> Result<()>  {
    let state = &mut ctx.accounts.state;
    let round = &mut ctx.accounts.round;

    require!(end_ts > start_ts, ErrorCode::InvalidTimestamps);

    round.id = state.round_count;
    round.start_ts = start_ts;
    round.end_ts = end_ts;
    round.resolved = false;
    round.prize_pool = 0;

    state.round_count += 1;

    Ok(())
}

pub fn submit_deck_inx(
    ctx: Context<SubmitDeck>,
    kard_ids: [Pubkey; 5],
) -> Result<()> {
    let clock = Clock::get()?;
    let round = &ctx.accounts.round;

    require!(
        clock.unix_timestamp >= round.start_ts &&
        clock.unix_timestamp < round.end_ts,
        ErrorCode::RoundNotActive
    );

    let submission = &mut ctx.accounts.submission;
    submission.user = ctx.accounts.user.key();
    submission.round = round.key();
    submission.kard_ids = kard_ids;
    submission.score = 0;
    submission.claimed = false;

    Ok(())
}

pub fn resolve_round_inx(
    ctx: Context<ResolveRound>,
    submission_keys: Vec<Pubkey>,
    scores: Vec<u64>,
) -> Result<()> {
    let round = &mut ctx.accounts.round;

    require!(!round.resolved, ErrorCode::RoundAlreadyResolved);
    require!(submission_keys.len() == scores.len(), ErrorCode::InvalidSubmission);

    for (submission_key, score) in submission_keys.iter().zip(scores.iter()) {
        let account_info = ctx.remaining_accounts
            .iter()
            .find(|a| a.key == submission_key)
            .ok_or(ErrorCode::InvalidSubmission)?;
        
        let mut submission_data = account_info.try_borrow_mut_data()?;
        let data_slice = &mut *submission_data;
        let mut submission = DeckSubmission::try_deserialize(&mut &data_slice[..])?;
        submission.score = *score;
        submission.try_serialize(&mut &mut data_slice[..])?;
    }

    round.resolved = true;
    Ok(())
}

pub fn claim_reward_inx(ctx: Context<ClaimReward>, amount: u64) -> Result<()> {
    let submission = &mut ctx.accounts.submission;
    let round = &ctx.accounts.round;

    require!(round.resolved, ErrorCode::RoundNotResolved);
    require!(!submission.claimed, ErrorCode::AlreadyClaimed);

    **ctx.accounts.treasury.to_account_info().try_borrow_mut_lamports()? -= amount;
    **ctx.accounts.user.to_account_info().try_borrow_mut_lamports()? += amount;

    submission.claimed = true;
    Ok(())
}
