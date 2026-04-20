use anchor_lang::prelude::*;
// use anchor_lang::system_program::{transfer, Transfer};
mod constants;
mod error;

pub mod instructions;
pub use instructions::*;
pub mod state;
pub use state::*;

pub mod stature_cpi; // This "activates" the file
pub use stature_cpi::*; // This makes the helper functions accessible

#[cfg(test)]
mod tests;

// 1. 🛑 Anti-abuse layer

// Right now:
// 👉 A company can still give +1M honor

// You’ll eventually need:

// Rate limits
// Stake/slashing
// Or weighting system

// 3. 🏆 Program reputation

// Eventually:

// Not all companies should have equal weight

// Think:

// Uber driver vs random user rating

declare_id!("9VFHpUQnHsG94AKzGfzf4mAeunxcQw8G9am6FfVEBVZb");

#[program]
pub mod stature {
    use super::*;

    // amins management
    pub fn create_admin(ctx: Context<InitConfig>) -> Result<()> {
        instructions::register_admin::init_config(ctx)
    }

    pub fn withdraw_funds(ctx: Context<WithdrawFees>, amount: u64) -> Result<()> {
        instructions::withdraw::withdraw_fees(ctx, amount)
    }

    pub fn create_program(ctx: Context<RegisterProgram>, name: String) -> Result<()> {
        instructions::register_program::register_program(ctx, name)
    }

    pub fn create_user(ctx: Context<RegisterUser>, name: String) -> Result<()> {
        instructions::register_user::register_user(ctx, name)
    }

    pub fn update_admin(ctx: Context<ManageAdmin>, new_admin: Pubkey) -> Result<()> {
        instructions::manage_admin::manage_admin(ctx, new_admin)
    }

    // manage program
    pub fn update_program_record_cap(ctx: Context<ManageProgram>, record_cap: u64) -> Result<()> {
        instructions::manage_program::manage_program_record_cap(ctx, record_cap)
    }
    pub fn update_program_stature(ctx: Context<ManageProgram>, new_stature: i64) -> Result<()> {
        instructions::manage_program::manage_program_stature(ctx, new_stature)
    }
    pub fn update_program_suspension(
        ctx: Context<ManageProgram>,
        is_suspended: bool,
    ) -> Result<()> {
        instructions::manage_program::manage_program_suspension(ctx, is_suspended)
    }
    pub fn update_program_verified_status(ctx: Context<ManageProgram>) -> Result<()> {
        instructions::manage_program::manage_program_verified_status(ctx)
    }
    pub fn update_program_weight(ctx: Context<ManageProgram>, weight: u16) -> Result<()> {
        instructions::manage_program::manage_program_weight(ctx, weight)
    }

    pub fn update_user_suspension(ctx: Context<ManageUser>, is_suspended: bool) -> Result<()> {
        instructions::manage_user::manage_user_suspension(ctx, is_suspended)
    }

    pub fn update_user_stature(
        ctx: Context<UpdateUserStatureCPI>,
        tx_value_lamports: i64,
        memo: String,
    ) -> Result<()> {
        instructions::update_stature::update_user_stature_via_cpi(ctx, tx_value_lamports, memo)
    }
}
