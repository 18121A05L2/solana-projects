use anchor_lang::prelude::*;

declare_id!("4fmNyKZSqhayxX1Vc7M6kqH4WNHrpMPKk4vNtpsczbtF");

#[program]
pub mod stablecoin {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
