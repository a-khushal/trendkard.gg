use anchor_lang::prelude::*;

declare_id!("AxjdRn3AkkhJBEwAJQogRWuw6hKK8fnjQUGLqT1RWJEn");

#[program]
pub mod trendkard_gg {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
