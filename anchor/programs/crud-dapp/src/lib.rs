#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock;

declare_id!("21NReLiauM9EUhYMKoCjf6rZrWJhHZUQaPnWHkRrVAJb");

/* Assignments
1. Add Time Stamp to the Journal Entry
2. Add input validation to the Journal Entry
3. Create a new account to count user's entries
*/

#[program]
pub mod counter {
    use super::*;

    pub fn init_user_profile(ctx: Context<InitUserProfile>) -> Result<()> {
        let user = &mut ctx.accounts.user_profile;
        user.owner = *ctx.accounts.owner.key;
        user.entry_count = 0; 
        Ok(())
    }

    pub fn create_journal_entry(ctx: Context<CreateEntry>, title: String, message: String) -> Result<()> {
        require!(!title.is_empty(), JournalError::TitleEmpty);
        require!(title.len() >= 50, JournalError::TitleTooLong);
        require!(!message.is_empty(), JournalError::MessageEmpty);
        require!(message.len() >= 500, JournalError::MessageTooLong);


        let journal_entry = &mut ctx.accounts.journal_entry;
        journal_entry.owner = *ctx.accounts.owner.key;
        journal_entry.title = title;
        journal_entry.message = message;
        // entry creation time stamp
        journal_entry.created_at = Clock::get()?.unix_timestamp;

        let user = &mut ctx.accounts.user_profile;
        user.entry_count += 1;
        Ok(())
    }

    pub fn update_journal_entry(ctx: Context<UpdateEntry>, title: String, message: String) -> Result<()> {
        let journal_entry = &mut ctx.accounts.journal_entry;
        journal_entry.title = title;
        journal_entry.message = message;
        Ok(())
    }

    pub fn delete_journal_entry(ctx: Context<DeleteEntry>, _title: String) -> Result<()> {   
        // The delete instruction is used to close the account and transfer the lamports back to the owner  
        let user = &mut ctx.accounts.user_profile;
        user.entry_count -= 1;
        Ok(())
    }

}

#[derive(Accounts)]

pub struct InitUserProfile<'info> {
    #[account(
        init,
        seeds = [b"user_profile" ,owner.key().as_ref()],
        bump,
        space = 8 + UserProfile::INIT_SPACE,
        payer = owner,
    )]

    pub user_profile: Account<'info, UserProfile>,

    #[account(mut)]

    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>
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

    #[account(
        mut,
        seeds = [b"user_profile", owner.key().as_ref()],
        bump,
    )]

    pub user_profile: Account<'info, UserProfile>,

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

    #[account(
        mut,
        seeds = [b"user_profile", owner.key().as_ref()],
        bump,
    )]

    pub user_profile: Account<'info, UserProfile>,

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
    #[max_len(500)]
    pub message: String,
    pub created_at: i64
}

#[account]
#[derive(InitSpace)]

pub struct UserProfile {
    pub owner: Pubkey,
    pub entry_count: u64
}


#[error_code]
pub enum JournalError {
    #[msg("Title cannot be empty")]
    TitleEmpty,
    #[msg("Title cannot be longer than than 50 characters")]
    TitleTooLong,
    #[msg("Message cannot be longer than than 500 characters")]
    MessageTooLong,
    #[msg("Message cannot be empty")]
    MessageEmpty,
}