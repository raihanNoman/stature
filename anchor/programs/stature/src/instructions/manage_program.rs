use crate::constants::MAX_COMPANY_WEIGHT;
use crate::error::ErrorCode;
use crate::state::{RegisteredProgram,Config};

use anchor_lang::prelude::*;


#[derive(Accounts)]
pub struct ManageProgram<'info> {
    pub admin: Signer<'info>,
    #[account(
        seeds = [b"config"],
        bump = config.bump,
        has_one = admin
    )]
    pub config: Account<'info, Config>,

    /// CHECK: This is the address we use to derive the PDA seeds
    pub target_program: UncheckedAccount<'info>,

    #[account(
        mut, 
        seeds = [b"registered_program", target_program.key().as_ref()], 
        bump = registered_program.bump, 
    )]
    pub registered_program: Account<'info, RegisteredProgram>,
}


pub fn manage_program_suspension(ctx: Context<ManageProgram>, is_suspended: bool) -> Result<()> {
    let registered_program = &mut ctx.accounts.registered_program;
    registered_program.is_suspended = is_suspended;

    msg!("🟢 registered_program {} is_suspended: {}", registered_program.name, registered_program.is_suspended );
    Ok(())
}

pub fn manage_program_weight(ctx: Context<ManageProgram>, new_weight: u16 ) -> Result<()> { 
    
    require!(new_weight > 0 && new_weight <= MAX_COMPANY_WEIGHT, ErrorCode::InvalidWeight);

    let registered_program = &mut ctx.accounts.registered_program;

    // new-weight 10 = 1.0x, 50 = 5.0x
    // As the "World Record" judge, you set the weight based on 
    // their contribution to the "Wealth of Nations"
    // todo: registered_program.weight = f(total_positive_raw_stature_given, trust_score, age)
    registered_program.weight = new_weight; 
    
    msg!("🟢 updated registered_program: {} weight: {}", registered_program.name, registered_program.weight);
    Ok(())
}

pub fn manage_program_record_cap(
    ctx: Context<ManageProgram>, 
    new_record_cap: u64 // 10 = 1.0x, 50 = 5.0x
) -> Result<()> {
    let registered_program = &mut ctx.accounts.registered_program;
    let old_cap  = registered_program.max_record_cap;

    registered_program.max_record_cap = new_record_cap; // todo: registered_program.weight = f(total_positive_raw_stature_given, trust_score, age)
    
    msg!("🟢 registered_program {} max record cap updated from {} to {}", registered_program.name, old_cap, new_record_cap);
    Ok(())
}


pub fn manage_program_verified_status(ctx: Context<ManageProgram>) -> Result<()> {
    let registered_program = &mut ctx.accounts.registered_program;
    registered_program.is_verified = true;
    registered_program.approved_by = ctx.accounts.admin.key();// The admin who verifies it becomes the recorded admin for this registered_program

    msg!("✅ success: registered_program {} is now verified. registered_program rep {} can now update user statures. registered_program can now be monitored, suspended, and updated by admin {}", registered_program.name,  registered_program.key(), registered_program.approved_by);
    Ok(())
}

// used to punish registered_program for bad behaviour
pub fn manage_program_stature(ctx: Context<ManageProgram>, new_stature: i64) -> Result<()> {
    let registered_program = &mut ctx.accounts.registered_program;
    registered_program.stature = new_stature;

    msg!("🟢 updated registered_program: {} stature: {}", registered_program.name, registered_program.stature);
    Ok(())
}




