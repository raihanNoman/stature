use anchor_lang::prelude::*;
use anchor_lang::solana_program;
use anchor_lang::system_program::{transfer, Transfer};
use crate::state::{ProgramUserState, RegisteredProgram, User};
pub use crate::program::Stature;

pub const STATURE_UPDATE_FEE: u64 = 1_000_000;

#[derive(Accounts)]
pub struct StatureUpdateBundle<'info> {
    #[account(mut)]
    pub stature_user: Account<'info, User>,
    #[account(mut)]
    pub registered_program: Account<'info, RegisteredProgram>,
    #[account(mut)]
    pub program_user_state: Account<'info, ProgramUserState>,
    #[account(mut)]
    pub stature_record: Signer<'info>,
  
    /// CHECK: The Vault PDA receiving the protocol fee
    #[account(
        mut,
        seeds = [b"stature_vault"],
        bump 
    )]
    pub stature_vault: UncheckedAccount<'info>,
    pub stature_program: Program<'info, Stature>,
    pub system_program: Program<'info, System>,
}

pub fn invoke_stature_update<'info>(
    bundle: &StatureUpdateBundle<'info>,
    payer: AccountInfo<'info>,
    target_program: AccountInfo<'info>, 
    source_account: AccountInfo<'info>, 
    tx_value: i64,
    memo: String,
) -> Result<()> {
    // 1. Handle Protocol Fee Transfer
    let fee_accounts = Transfer {
        from: payer.clone(),
        to: bundle.stature_vault.to_account_info(),
    };
    let fee_ctx = CpiContext::new(bundle.system_program.to_account_info(), fee_accounts);
    transfer(fee_ctx, STATURE_UPDATE_FEE)?;

    // 2. Prepare Instruction Data
    // We use crate::instruction because we are inside the program
    let mut data = anchor_lang::InstructionData::data(&crate::instruction::UpdateUserStature {
        tx_value_lamports: tx_value,
        memo,
    });

    // 8-byte discriminator for "update_user_stature"
    let mut instruction_data = crate::instruction::UpdateUserStature::DISCRIMINATOR.to_vec();
    instruction_data.append(&mut data);

    // 3. Map the accounts correctly for the target instruction
    // IMPORTANT: These must be in the EXACT order your update_user_stature instruction expects
    let accounts = vec![
        AccountMeta::new(payer.key(), true),
        AccountMeta::new_readonly(target_program.key(), false),
        AccountMeta::new(bundle.registered_program.key(), false),
        AccountMeta::new_readonly(payer.key(), false), // user_wallet
        AccountMeta::new(bundle.stature_user.key(), false),
        AccountMeta::new(source_account.key(), false), // registered_program_source_account
        AccountMeta::new(bundle.program_user_state.key(), false),
        AccountMeta::new(bundle.stature_record.key(), true),
        AccountMeta::new_readonly(bundle.system_program.key(), false),
    ];

    let ix = solana_program::instruction::Instruction {
        program_id: crate::ID,
        accounts,
        data: instruction_data,
    };

    // 4. Invoke
    solana_program::program::invoke(
        &ix,
        &[
            payer,
            target_program,
            bundle.registered_program.to_account_info(),
            bundle.stature_user.to_account_info(),
            source_account,
            bundle.program_user_state.to_account_info(),
            bundle.stature_record.to_account_info(),
            bundle.system_program.to_account_info(),
        ],
    )?;

    Ok(())
}