use crate::state::Config;

use anchor_lang::prelude::*;

//1. The "Two-Step Transfer" Suggestion
//Currently, if you accidentally paste the wrong address into new_admin (e.g., a random exchange deposit address or a typo), you lose control of your program instantly and forever. There is no "undo" button.
//In professional DeFi and Protocol design, we often use a two-step transfer:
//Admin sets a pending_admin.
// The pending_admin must sign a transaction to claim_admin status.
// This proves the new address is active and controlled by someone before the old admin is removed.
//If you want to keep it simple (one-step), your code is perfect as-is. Just be extremely careful when calling it!

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
