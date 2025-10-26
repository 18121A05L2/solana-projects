#![allow(unexpected_cfgs)]
#![allow(deprecated)]

use anchor_lang::prelude::*;

declare_id!("D633YF6RQ59eQmV5xhTFZL9SJ21CU2iYNWSihUQSWB1b");

#[program]
pub mod crud {
    use super::*;

    pub fn create_journal_entry(
        ctx: Context<CreateEntry>,
        title: String,
        message: String,
    ) -> Result<()> {
        let journal_entry = &mut ctx.accounts.journal_entry;
        journal_entry.title = title;
        journal_entry.message = message;
        journal_entry.owner = *ctx.accounts.owner.key;
        Ok(())
    }

    pub fn update_journal_entry(
        ctx: Context<UpdateEntry>,
        _title: String,
        message: String,
    ) -> Result<()> {
        let journal_entry = &mut ctx.accounts.journal_entry;
        journal_entry.message = message;
        Ok(())
    }

    pub fn delete_journal_entry(_ctx: Context<DeleteEntry>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(title : String)]
pub struct CreateEntry<'info> {
    #[account(init , seeds=[title.as_bytes(), owner.key().as_ref()], bump, payer = owner , space = 8 + JournalEntry::INIT_SPACE )]
    pub journal_entry: Account<'info, JournalEntry>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(title : String)]
pub struct UpdateEntry<'info> {
    #[account(mut , seeds=[title.as_bytes(), owner.key().as_ref()], bump, realloc = 8 + JournalEntry::INIT_SPACE , realloc::payer = owner , realloc::zero = true )]
    pub journal_entry: Account<'info, JournalEntry>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(title : String)]
pub struct DeleteEntry<'info> {
    #[account(mut,  seeds=[title.as_bytes(), owner.key().as_ref()], bump, close = owner )]
    pub journal_entry: Account<'info, JournalEntry>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct JournalEntry {
    pub owner: Pubkey,
    #[max_len(50)]
    pub title: String,
    #[max_len(50)]
    pub message: String,
}
