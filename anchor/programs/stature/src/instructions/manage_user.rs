use crate::error::ErrorCode;
use crate::state::{Config, StatureUser};

use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ManageUser<'info> {
    pub admin: Signer<'info>,

    #[account(
        seeds = [b"config"],
        bump = config.bump,
        has_one = admin
    )]
    pub config: Account<'info, Config>,


    #[account(
        mut, 
        seeds = [b"user", user.wallet.as_ref()], 
        bump = user.bump
    )]
    pub user: Account<'info, StatureUser>,
}



pub fn manage_user_suspension(ctx: Context<ManageUser>, is_suspended: bool) -> Result<()> {
    require!(ctx.accounts.config.admin == ctx.accounts.admin.key(), ErrorCode::AdminActionOnly);


    let user = &mut ctx.accounts.user;
    user.is_suspended = is_suspended;

    msg!("🟢 user {} is_suspended: {}", user.key(), user.is_suspended );
    Ok(())
}
