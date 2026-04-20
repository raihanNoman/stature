use crate::constants::ANCHOR_DISCRIMINATOR;
use crate::error::ErrorCode;
use crate::state::Config;

use anchor_lang::prelude::*;
#[derive(Accounts)]
pub struct WithdrawFees<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        seeds = [b"config"],
        bump = config.bump,
        has_one = admin, // Security: Ensures ctx.accounts.admin == config.admin
    )]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        seeds = [b"stature_vault"],
        bump, // You might want to store this bump in Config too
    )]
    pub stature_vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

pub fn withdraw_fees(ctx: Context<WithdrawFees>, amount: u64) -> Result<()> {
    let vault = &ctx.accounts.stature_vault;
    let admin = &ctx.accounts.admin;

    // Safety check: Don't try to withdraw more than the vault has
    let rent_balance = Rent::get()?.minimum_balance(0);
    let withdrawable = vault.lamports().saturating_sub(rent_balance);

    require!(amount <= withdrawable, ErrorCode::InsufficientFunds);

    // To move SOL from a PDA, we "subtract" from one and "add" to the other
    // Access the lamports via the account_info() and use dereferencing properly
    **ctx
        .accounts
        .stature_vault
        .to_account_info()
        .try_borrow_mut_lamports()? -= amount;
    **ctx
        .accounts
        .admin
        .to_account_info()
        .try_borrow_mut_lamports()? += amount;

    msg!(
        "Successfully withdrew {} lamports to admin {}",
        amount,
        admin.key()
    );
    Ok(())
}
