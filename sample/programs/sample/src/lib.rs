use anchor_lang::prelude::*;

declare_id!("FKULT8yMhNW5XM4vG5GLmhw28z8KRSHTEk4wc895Ae5u");

#[program]
pub mod sample {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
