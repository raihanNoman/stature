use crate::constants::ANCHOR_DISCRIMINATOR;
use crate::state::RegisteredProgram;

use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct RegisterProgram<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    /// The Program ID you are whitelisting
    /// CHECK: This is just a pubkey reference
    pub target_program: UncheckedAccount<'info>,

    #[account(
        init,
        payer = payer,
        space = ANCHOR_DISCRIMINATOR + RegisteredProgram::INIT_SPACE,
        seeds = [b"registered_program", target_program.key().as_ref()],
        bump
    )]
    pub registered_program: Account<'info, RegisteredProgram>,

    pub system_program: Program<'info, System>,
}

/// Admin registers an external Program ID (e.g., Contract282) into the Stature Protocol
pub fn register_program(ctx: Context<RegisterProgram>, name: String) -> Result<()> {
    let registered_program = &mut ctx.accounts.registered_program;
    registered_program.program_id = ctx.accounts.target_program.key();
    registered_program.name = name;
    registered_program.weight = 1; // Default weight
    registered_program.is_verified = false;
    registered_program.is_suspended = false;
    registered_program.since = Clock::get()?.unix_timestamp;
    registered_program.bump = ctx.bumps.registered_program;

    msg!(
        "✅ Program {} registered in Stature Protocol",
        registered_program.program_id
    );
    Ok(())
}
