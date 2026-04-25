use crate::constants::ANCHOR_DISCRIMINATOR;
use crate::state::RegisteredProgram;

use anchor_lang::prelude::*;
use anchor_lang::solana_program::bpf_loader_upgradeable;

#[derive(Accounts)]
pub struct RegisterProgram<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

/// CHECK: We verify this is a program by deriving its ProgramData PDA below.
    /// If this isn't a valid Program ID, the program_data account derivation will fail.
    pub target_program: UncheckedAccount<'info>,

    /// The ProgramData account containing the upgrade authority
    #[account(
        seeds = [target_program.key().as_ref()],
        seeds::program = bpf_loader_upgradeable::id(),
        bump,
    )]
    pub program_data: Account<'info, ProgramData>,

    /// The Program ID you are whitelisting
    /// CHECK: This is just a pubkey reference
    // pub target_program: UncheckedAccount<'info>,

    #[account(
        init,
        payer = authority,
        space = ANCHOR_DISCRIMINATOR + RegisteredProgram::INIT_SPACE,
        seeds = [b"registered_program", target_program.key().as_ref()],
        bump, 
        // CRITICAL CHECK: Does the signer match the program's upgrade authority?
        // Ensures the signer is the one who actually owns the program
        constraint = program_data.upgrade_authority_address == Some(authority.key())
    )]
    pub registered_program: Account<'info, RegisteredProgram>,

    pub system_program: Program<'info, System>,
}

/// Admin registers an external Program ID (e.g., Contract282) into the Stature Protocol
pub fn register_program(ctx: Context<RegisterProgram>, name: String) -> Result<()> {
    let registered_program = &mut ctx.accounts.registered_program;
    registered_program.target_program = ctx.accounts.target_program.key();
    registered_program.authority = ctx.accounts.authority.key();
    registered_program.name = name;
    registered_program.weight = 1; // Default weight
    registered_program.is_verified = false;
    registered_program.is_suspended = false;
    registered_program.since = Clock::get()?.unix_timestamp;
    registered_program.bump = ctx.bumps.registered_program;

    msg!(
        "✅ Program {} registered in Stature Protocol",
        registered_program.target_program
    );
    Ok(())
}
