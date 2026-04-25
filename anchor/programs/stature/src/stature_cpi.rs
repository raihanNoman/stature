pub use crate::program::Stature;
use crate::state::{ProgramUserState, RegisteredProgram, StatureRecord, StatureUser};
use anchor_lang::prelude::*;
use anchor_lang::solana_program;

pub const STATURE_UPDATE_FEE: u64 = 1_000_000; // 0.001 SOL

pub use crate::instruction::UpdateUserStature;
pub use crate::UpdateUserStatureCPI;

#[derive(Accounts)]
pub struct StatureUpdateBundle<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    /// CHECK: The executable program ID
    pub target_program: UncheckedAccount<'info>,
    #[account(mut)]
    pub registered_program: Account<'info, crate::state::RegisteredProgram>,
    /// CHECK: The user's main wallet
    pub user_wallet: SystemAccount<'info>,
    #[account(mut)]
    pub user: Account<'info, crate::state::StatureUser>,
    /// CHECK: The data account from the calling program (source)
    #[account(mut)]
    pub registered_program_source_account: UncheckedAccount<'info>,
    #[account(mut)]
    pub program_user_state: Account<'info, crate::state::ProgramUserState>,
    #[account(mut)]
    pub record: Account<'info, crate::state::StatureRecord>,

    /// CHECK: Stature Protocol Fee Vault
    #[account(mut, seeds = [b"stature_vault"], bump)]
    pub stature_vault: UncheckedAccount<'info>,

    pub stature_program: Program<'info, Stature>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct StatureUpdateBundleOld<'info> {
    /// CHECK: Expects a Stature User Account
    pub stature_user: Account<'info, StatureUser>, //
    /// CHECK: Expects a RegisteredProgram Account
    pub registered_program: Account<'info, RegisteredProgram>,
    /// CHECK: Expects a ProgramUserState Account
    pub program_user_state: Account<'info, ProgramUserState>,
    /// CHECK: Must be a Signer
    pub stature_record: Account<'info, StatureRecord>,
    /// CHECK: The Stature Fee Vault
    ///
    ///
    #[account(mut, seeds = [b"stature_vault"], bump)]
    pub stature_vault: UncheckedAccount<'info>,
    /// CHECK: The Vault PDA receiving the protocol fee
    // #[account(
    //     mut,
    //     seeds = [b"stature_vault"],
    //     bump
    // )]
    // pub stature_vault: UncheckedAccount<'info>,
    pub stature_program: Program<'info, Stature>,
    pub system_program: Program<'info, System>,
}

// use crate::state::{ProgramUserState, RegisteredProgram, StatureUser};

// pub struct StatureUpdateBundle<'info> {
//     #[account(mut)]
//     pub stature_user: Account<'info, StatureUser>,
//     #[account(mut)]
//     pub registered_program: Account<'info, RegisteredProgram>,
//     #[account(mut)]
//     pub program_user_state: Account<'info, ProgramUserState>,
//     #[account(mut)]
//     pub stature_record: Signer<'info>,

/// CHECK: The Vault PDA receiving the protocol fee
//     #[account(
//         mut,
//         seeds = [b"stature_vault"],
//         bump
//     )]
//     pub stature_vault: UncheckedAccount<'info>,
//     pub stature_program: Program<'info, Stature>,
//     pub system_program: Program<'info, System>,
// }

pub fn invoke_stature_update<'info>(
    bundle: &StatureUpdateBundle<'info>,
    tx_value: i64,
    memo: String,
) -> Result<()> {
    anchor_lang::system_program::transfer(
        CpiContext::new(
            *bundle.system_program.key,
            anchor_lang::system_program::Transfer {
                from: bundle.signer.to_account_info(),
                to: bundle.stature_vault.to_account_info(),
            },
        ),
        STATURE_UPDATE_FEE,
    )?;

    // 2. Prepare CPI Accounts for UpdateUserStatureCPI
    // These MUST match the order and names of your actual instruction struct
    let cpi_accounts = UpdateUserStatureCPI {
        signer: bundle.signer.clone(),
        target_program: bundle.target_program.clone(),
        registered_program: bundle.registered_program.clone(),
        user_wallet: bundle.user_wallet.clone(), // This now matches because bundle uses SystemAccount
        user: bundle.user.clone(),
        registered_program_source_account: bundle.registered_program_source_account.clone(),
        program_user_state: bundle.program_user_state.clone(),
        record: bundle.record.clone(),
        system_program: bundle.system_program.clone(),
    };

    // 3. Create the Instruction manually using Anchor's helpers
    let mut data = anchor_lang::InstructionData::data(&crate::instruction::UpdateUserStature {
        tx_value_lamports: tx_value,
        memo,
    });
    let mut ix_data = crate::instruction::UpdateUserStature::DISCRIMINATOR.to_vec();
    ix_data.append(&mut data);

    // 4. Map the AccountInfos
    let infos = cpi_accounts.to_account_infos();
    let metas = cpi_accounts.to_account_metas(None);

    let ix = anchor_lang::solana_program::instruction::Instruction {
        program_id: crate::ID,
        accounts: metas,
        data: ix_data,
    };

    // 5. Invoke (Use invoke_signed if the caller is a PDA)
    anchor_lang::solana_program::program::invoke(&ix, &infos)?;

    Ok(())
}

pub fn invoke_stature_update_old<'info>(
    bundle: &StatureUpdateBundleOld<'info>,
    payer: AccountInfo<'info>,
    target_program: AccountInfo<'info>,
    source_account: AccountInfo<'info>,
    tx_value: i64,
    memo: String,
) -> Result<()> {
    anchor_lang::system_program::transfer(
        CpiContext::new(
            *bundle.system_program.key,
            anchor_lang::system_program::Transfer {
                from: payer.clone(),
                to: bundle.stature_vault.to_account_info(),
            },
        ),
        STATURE_UPDATE_FEE,
    )?;

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
        AccountMeta::new(bundle.stature_record.key(), false),
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

// The "Interface" definition
pub trait StatureInterface<'info> {
    fn get_bundle(&self) -> StatureUpdateBundleOld<'info>;
    fn get_payer(&self) -> AccountInfo<'info>;
}

// Update the helper to accept ANY type that implements your interface
pub fn invoke_stature_with_interface<'info, T: StatureInterface<'info>>(
    ctx_struct: &T,
    target_program: AccountInfo<'info>,
    source_account: AccountInfo<'info>,
    tx_value: i64,
    memo: String,
) -> Result<()> {
    let bundle = ctx_struct.get_bundle();
    let payer = ctx_struct.get_payer();

    // Call your existing invoke logic
    invoke_stature_update_old(
        &bundle,
        payer,
        target_program,
        source_account,
        tx_value,
        memo,
    )
}
