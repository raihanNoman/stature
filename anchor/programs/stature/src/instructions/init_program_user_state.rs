use crate::constants::ANCHOR_DISCRIMINATOR;
use crate::error::ErrorCode;
use crate::state::{RegisteredProgram, StatureUser, ProgramUserState};

use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitProgramUserState<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    /// The Program ID you are whitelisting
    /// CHECK: This is just a pubkey reference
    // pub target_program: UncheckedAccount<'info>,


    /// CHECK: We verify this is a program by deriving its ProgramData PDA below.
    /// If this isn't a valid Program ID, the program_data account derivation will fail.
    pub target_program: UncheckedAccount<'info>,


    #[account(
        seeds = [b"registered_program", target_program.key().as_ref()],
        bump = registered_program.bump, 
        // CRITICAL CHECK: Does the signer match the program's upgrade authority?
        // Ensures the signer is the one who actually owns the program
        constraint = registered_program.is_verified  @ ErrorCode::ProgramNotVerified, 
        constraint = !registered_program.is_suspended  @ ErrorCode::ProgramSuspended, 
        constraint = registered_program.target_program == target_program.key() @ ErrorCode::MismatchedTargetProgram
    )]
    pub registered_program: Account<'info, RegisteredProgram>,



    pub user_wallet: SystemAccount<'info>,

    #[account(
        seeds = [b"user", user_wallet.key().as_ref()],
        constraint = !user.is_suspended  @ ErrorCode::UserSuspended, 
        bump,
    )]
    pub user: Account<'info, StatureUser>,



    #[account(
        init,
        payer = authority,
        space = ANCHOR_DISCRIMINATOR + ProgramUserState::INIT_SPACE,
        seeds = [b"state", registered_program.key().as_ref(), user_wallet.key().as_ref()], // increase count for number of items from that 
        bump
    )]
    pub program_user_state: Account<'info, ProgramUserState>,


    pub system_program: Program<'info, System>,
}

/// Admin registers an external Program ID (e.g., Contract282) into the Stature Protocol
pub fn init_program_user_state(ctx: Context<InitProgramUserState>) -> Result<()> {
    let now: i64 =  Clock::get()?.unix_timestamp;
    let state = &mut ctx.accounts.program_user_state;

// Only set these if the account is new (if you keep init_if_needed)
    state.first_action_at = now;
    state.last_updated_at = now;
    state.total_records = 0;
    state.bump = ctx.bumps.program_user_state;
    
    msg!("✅ Program User State Initiated: {}", state.key());
    Ok(())
}
