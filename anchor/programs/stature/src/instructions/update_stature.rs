use crate::calculate_stature_gain;
use crate::constants::{ANCHOR_DISCRIMINATOR};
use crate::error::ErrorCode;
use crate::state::{ ProgramUserState, RegisteredProgram, StatureRecord, StatureUser};

use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct UpdateUserStatureCPI<'info> {
    /// The program calling this MUST sign/authorize via its PDA or specific key
    /// In a Program-to-Program model, this is usually a PDA from the calling program
    #[account(mut)] // <--- ADD THIS LINE
    pub payer: Signer<'info>, // Can be the User OR the Program PDA


    /// CHECK: This is the address we use to derive the PDA seeds
    pub target_program: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [b"registered_program", target_program.key().as_ref()],
        owner = target_program.key() @ ErrorCode::InvalidSourceOwner, 
        bump = registered_program.bump,
        constraint = !registered_program.is_suspended@ ErrorCode::ProgramSuspended, 
        constraint = registered_program.is_verified @ ErrorCode::ProgramNotVerified,
    )]
    pub registered_program: Account<'info, RegisteredProgram>,


    pub user_wallet: SystemAccount<'info>,

    #[account(
        init_if_needed, 
        payer = payer,
        space = ANCHOR_DISCRIMINATOR + StatureUser::INIT_SPACE,
        seeds = [b"user", user_wallet.key().as_ref()], 
        bump,
    )]
    pub user: Account<'info, StatureUser>,


    /// CHECK: Validated in instruction logic //? add contraints ?
    #[account(mut)]
    pub registered_program_source_account: UncheckedAccount<'info>, // ! unchecked account 

    #[account(
        init_if_needed,
        payer = payer,
        space = ANCHOR_DISCRIMINATOR + ProgramUserState::INIT_SPACE,
        seeds = [b"state", registered_program.key().as_ref(), user_wallet.key().as_ref()], // increase count for number of items from that 
        bump
    )]
    pub program_user_state: Account<'info, ProgramUserState>,

    #[account(
        init,
        payer = payer,
        space = ANCHOR_DISCRIMINATOR + StatureRecord::INIT_SPACE,
        seeds = [
            b"record", 
            user_wallet.key().as_ref(), 
            registered_program.key().as_ref(), // Added for extra isolation
            registered_program_source_account.key().as_ref(), 
        ],
        bump
    )]
    pub record: Account<'info, StatureRecord>,

    pub system_program: Program<'info, System>,
}



pub fn update_user_stature_via_cpi(
    ctx: Context<UpdateUserStatureCPI>,
    tx_value_lamports: i64,
    memo: String, 
) -> Result<()> {
    let now: i64 =  Clock::get()?.unix_timestamp;
    let registered_program = &mut ctx.accounts.registered_program;
    let user = &mut ctx.accounts.user;
    let source_account =  &mut ctx.accounts.registered_program_source_account;
    let record = &mut ctx.accounts.record;
    let program_user_state = &mut ctx.accounts.program_user_state;

 

    if program_user_state.total_records == 0 {
        program_user_state.first_action_at = now;
    }


    if user.wallet == Pubkey::default() {
        let mut name = format!("new {} user", registered_program.name);

        if name.len() > 32 {
            name = "new user".to_string();
        }

        user.wallet = ctx.accounts.user_wallet.key();
        user.name = name;
        user.stature = 0;
        user.total_records = 0;
        user.since = now;
        user.is_suspended = false;
        user.bump = ctx.bumps.user;
    }

    if user.first_action_at == 0 {
        user.first_action_at = now;
    }

    // 1. Logic Guards
    require!(!user.is_suspended, ErrorCode::UserSuspended);

    let adjusted_stature = calculate_stature_gain(
        tx_value_lamports,
        registered_program.weight,
        user.total_records,
        user.first_action_at,
        now,
    )?;

    // 3. Update State


    if tx_value_lamports > 0 {
        user.total_positive_tx = user.total_positive_tx
            .checked_add(tx_value_lamports as u64)
            .ok_or(ErrorCode::Overflow)?; // Convert Option to Result

        registered_program.total_positive_tx = registered_program.total_positive_tx
            .checked_add(tx_value_lamports)
            .ok_or(ErrorCode::Overflow)?;
    } else {
        let penalty_abs = tx_value_lamports.abs() as u64;
        
        user.total_negative_tx = user.total_negative_tx
            .checked_add(penalty_abs)
            .ok_or(ErrorCode::Overflow)?;

        registered_program.total_negative_tx = registered_program.total_negative_tx
            .checked_add(tx_value_lamports.abs())
            .ok_or(ErrorCode::Overflow)?;
    }

    user.stature = user
        .stature
        .checked_add(adjusted_stature)
        .ok_or(ErrorCode::Overflow)?;

    registered_program.stature = registered_program
        .stature
        .checked_add(adjusted_stature)
        .ok_or(ErrorCode::Overflow)?;

    // 4. Record keeping

    record.user = user.wallet.key(); 
    record.registered_program = registered_program.key();
    record.registered_program_source_account = source_account.key();
    record.weight = registered_program.weight;

    record.tx_value = tx_value_lamports; // Store the raw input
    record.stature = adjusted_stature;
    record.memo = memo.clone();
    record.bump = ctx.bumps.record;
    record.user_record_idx = user.total_records;

    record.timestamp =now;
    program_user_state.last_updated_at = now;


    user.total_records += 1;

    registered_program.record_count += 1;
    program_user_state.total_records += 1;
    program_user_state.bump = ctx.bumps.program_user_state; // Ensure bump is saved

    emit!(UpdateStatureEvent {
        user_wallet: user.wallet,
        program: registered_program.key(),
        program_source_account: source_account.key(), 
        stature: adjusted_stature.try_into().map_err(|_| ErrorCode::Overflow)?,
        memo: memo.clone(), 
        timestamp: now, 
    });

    msg!("🟢 Stature Updated: {} stature for user {} (wallet: {} ) by program: {}", adjusted_stature, user.key(), user.wallet.key(),  registered_program.name);
    Ok(())
}



// for emiting event
#[event]
pub struct UpdateStatureEvent {
    pub user_wallet: Pubkey,
    pub program: Pubkey,
    pub program_source_account: Pubkey,
    pub stature: i64,

    pub memo: String, 
    pub timestamp: i64,
}

