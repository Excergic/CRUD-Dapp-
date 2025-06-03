#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("21NReLiauM9EUhYMKoCjf6rZrWJhHZUQaPnWHkRrVAJb");

#[program]
pub mod counter {
    use super::*;

    // all the instructions are in this #program macro

    pub fn create_journal_entry(ctx: Context<CreateEntry>, title: String, message: String) -> Result<()> {
        let journal_entry = &mut ctx.accounts.journal_entry;
        journal_entry.owner = *ctx.accounts.owner.key;
        journal_entry.title = title;
        journal_entry.message = message; 
        Ok(())
    }

    pub fn update_journal_entry(ctx: Context<UpdateEntry>, title: String, message: String) -> Result<()> {
        let journal_entry = &mut ctx.accounts.journal_entry;
        journal_entry.title = title;
        journal_entry.message = message;
        Ok(())
    }

    pub fn delete_journal_entry(_ctx: Context<DeleteEntry>, _title: String) -> Result<()> {   
        // The delete instruction is used to close the account and transfer the lamports back to the owner     
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(title: String)]

pub struct CreateEntry<'info> {
    #[account(
        init,
        seeds = [title.as_bytes(), owner.key().as_ref()],
        bump,
        space = 8 + JournalEntryState::INIT_SPACE,
        payer = owner,
    )]

    pub journal_entry: Account<'info, JournalEntryState>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(title: String)]

pub struct UpdateEntry<'info> {
    #[account(
        mut,
        seeds = [title.as_bytes(), owner.key().as_ref()],
        bump,
        realloc = 8 + JournalEntryState::INIT_SPACE,
        realloc::payer = owner,
        realloc::zero = true,
    )]

    pub journal_entry: Account<'info, JournalEntryState>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(title: String)]

pub struct DeleteEntry<'info> {
    #[account(
        mut,
        seeds = [title.as_bytes(), owner.key().as_ref()],
        bump,
        close = owner,
    )]

    pub journal_entry: Account<'info, JournalEntryState>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}
 
#[account]

// create an account for crud data (journal entry account)

// user need to pay for storage (each journal entry is a account) for that space needs to calculate

#[derive(InitSpace)]

pub struct JournalEntryState {
    pub owner: Pubkey,
    #[max_len(50)]
    pub title: String,
    #[max_len(280)]
    pub message: String,
}