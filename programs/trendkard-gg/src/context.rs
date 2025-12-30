use anchor_lang::prelude::*;
use crate::state::{State, Round, DeckSubmission};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8 + State::INIT_SPACE)]
    pub state: Account<'info, State>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateRound<'info> {
    #[account(mut, has_one = authority)]
    pub state: Account<'info, State>,
    #[account(init, payer = authority, space = 8 + Round::INIT_SPACE)]
    pub round: Account<'info, Round>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SubmitDeck<'info> {
    #[account()]
    pub round: Account<'info, Round>,
    #[account(
        init,
        payer = user,
        space = 8 + DeckSubmission::INIT_SPACE,
        seeds = [b"submission", round.key().as_ref(), user.key().as_ref()],
        bump
    )]
    pub submission: Account<'info, DeckSubmission>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct ResolveRound<'info> {
    #[account(mut)]
    pub round: Account<'info, Round>,
    #[account(has_one = authority)]
    pub state: Account<'info, State>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct ClaimReward<'info> {
    #[account()]
    pub round: Account<'info, Round>,
    #[account(mut)]
    pub submission: Account<'info, DeckSubmission>,
    #[account(mut)]
    pub treasury: SystemAccount<'info>,
    #[account(mut)]
    pub user: Signer<'info>,
}

