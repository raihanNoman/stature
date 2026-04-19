use crate::constants::ANCHOR_DISCRIMINATOR;
use crate::error::ErrorCode;
use crate::User;

use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct RegisterUser<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = ANCHOR_DISCRIMINATOR + User::INIT_SPACE,
        seeds = [b"user", owner.key().as_ref()],
        bump
    )]
    pub user: Account<'info, User>,

    pub system_program: Program<'info, System>,
}

/// Admin registers an external Program ID (e.g., Contract282) into the Stature Protocol
pub fn register_user(ctx: Context<RegisterUser>, name: String) -> Result<()> {
    require!(name.len() <= 32, ErrorCode::StringTooLong);

    let user = &mut ctx.accounts.user;
    user.wallet = ctx.accounts.owner.key();
    user.name = name;
    user.stature = 0;
    user.record_idx = 0;
    user.is_suspended = false;
    user.bump = ctx.bumps.user;

    msg!("✅ success: initialized user {} by {}", user.name, user.wallet);

    Ok(())
}
