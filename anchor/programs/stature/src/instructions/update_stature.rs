use crate::constants::{ANCHOR_DISCRIMINATOR};
use crate::error::ErrorCode;
use crate::state::{ ProgramUserState, RegisteredProgram, StatureRecord, User};

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
        bump = registered_program.bump,
        constraint = !registered_program.is_suspended@ ErrorCode::ProgramSuspended, 
        constraint = registered_program.is_verified @ ErrorCode::ProgramNotVerified,
    )]
    pub registered_program: Account<'info, RegisteredProgram>,


    pub user_wallet: SystemAccount<'info>,

    #[account(
        init_if_needed, 
        payer = payer,
        space = ANCHOR_DISCRIMINATOR + User::INIT_SPACE,
        seeds = [b"user", user_wallet.key().as_ref()], 
        bump,
    )]
    pub user: Account<'info, User>,


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
    amount: i64,
    nonce: u64,
) -> Result<()> {
    let registered_program = &mut ctx.accounts.registered_program;
    let user = &mut ctx.accounts.user;
    let source_account =  &mut ctx.accounts.registered_program_source_account;
    let record = &mut ctx.accounts.record;
    let integrator_user_state = &mut ctx.accounts.program_user_state;



    if user.wallet == Pubkey::default() {
        let mut name = format!("new {} user", registered_program.name);

        if name.len() > 32 {
            name = "new user".to_string();
        }

        user.wallet = ctx.accounts.user_wallet.key();
        user.name = name;
        user.stature = 0;
        user.record_idx = 0;
        user.is_suspended = false;
        user.bump = ctx.bumps.user;
    }
    
    // 1. Logic Guards
    require!(!user.is_suspended, ErrorCode::UserSuspended);
    require!(nonce > integrator_user_state.last_nonce,ErrorCode::InvalidNonce);

    // 2. Calculation Logic (Weighted by Program Authority)
    let weight_multiplier = registered_program.weight as i64;
    let weighted_honor = amount
        .checked_mul(weight_multiplier)
        .ok_or(ErrorCode::Overflow)?;

    // Anti-inflation scaling
    let value = user.stature.abs() as u64 + 1;
    let scale = (64 - value.leading_zeros()) as i64;
    let adjusted_honor = weighted_honor
        .checked_div(scale.max(1))
        .ok_or(ErrorCode::Overflow)?;

    // 3. Update State
    user.stature = user
        .stature
        .checked_add(adjusted_honor)
        .ok_or(ErrorCode::Overflow)?;

    // Update Integration metrics
    if amount > 0 {
        registered_program.total_positive_raw = registered_program
            .total_positive_raw
            .checked_add(amount)
            .ok_or(ErrorCode::Overflow)?;
        registered_program.stature = registered_program
            .stature
            .checked_add(adjusted_honor)
            .ok_or(ErrorCode::Overflow)?;
    }

    // 4. Record keeping
    integrator_user_state.last_nonce = nonce;
    integrator_user_state.last_updated_at = Clock::get()?.unix_timestamp;

    record.registered_program = registered_program.key();
    record.user = user.key();
    record.amount = adjusted_honor;
    record.timestamp = integrator_user_state.last_updated_at;

    user.record_idx += 1;


    emit!(UpdateStatureEvent {
        user: user.wallet,
        program: registered_program.key(),
        source: source_account.key(), 
        amount: adjusted_honor.try_into().map_err(|_| ErrorCode::Overflow)?,
        timestamp: record.timestamp, 
    });

    msg!("🟢 Stature Updated by Program: {}", registered_program.name);
    Ok(())
}



// for emiting event
#[event]
pub struct UpdateStatureEvent {
    pub user: Pubkey,
    pub program: Pubkey,
    pub source: Pubkey,
    pub amount: i64,
    pub timestamp: i64,
}

