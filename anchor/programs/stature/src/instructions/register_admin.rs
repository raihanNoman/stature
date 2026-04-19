use crate::constants::ANCHOR_DISCRIMINATOR;
use crate::error::ErrorCode;
use crate::state::Config;

use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitConfig<'info> {
    #[account(mut)]
    pub admin: Signer<'info>, // right now its just my wallet - // todo: upgrade to multi sig

    #[account(
        init,
        payer = admin,
        space = ANCHOR_DISCRIMINATOR + Config::INIT_SPACE,
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, Config>,

    pub system_program: Program<'info, System>,
}

pub fn init_config(ctx: Context<InitConfig>) -> Result<()> {
    let config = &mut ctx.accounts.config;

    require!(!config.is_initilized, ErrorCode::AlreadyInitialized);

    config.admin = ctx.accounts.admin.key();
    config.bump = ctx.bumps.config;
    config.is_initilized = true;

    msg!("✅ success: initialized admin keys for stature.network. Now {} can verify companies and update company status",config.admin,  );
    Ok(())
}
