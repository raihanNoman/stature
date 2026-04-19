use crate::state::Config;

use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ManageAdmin<'info> {
    #[account(mut)]
    pub admin: Signer<'info>, // current admin

    #[account(
        mut,
        seeds = [b"config"],
        bump = config.bump,
        has_one = admin // 🔒 only current admin can update
    )]
    pub config: Account<'info, Config>,
}

pub fn manage_admin(ctx: Context<ManageAdmin>, new_admin: Pubkey) -> Result<()> {
    let config = &mut ctx.accounts.config;
    let old_admin_key = config.admin;
    config.admin = new_admin;

    msg!(
        "✅ success: changed admin from {} to {}",
        old_admin_key,
        config.admin,
    );

    Ok(())
}
