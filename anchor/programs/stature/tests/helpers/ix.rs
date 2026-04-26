use anchor_lang::Key;
use anchor_litesvm::{AnchorContext, Instruction, Keypair, Pubkey, Signer};
//litesvm::solana_sdk::account::Account;
use litesvm_utils::TestHelpers;
use solana_sdk::signer::keypair;

use crate::helpers::ix_management::ix_manage_program_stature;
use crate::helpers::mock::mock_program_data;
use crate::helpers::setup::*;

use crate::stature::client;

pub fn ix_init_config(ctx: &mut AnchorContext, admin: &Pubkey) -> Instruction {
    let config_pda = get_config_pda();
    let stature_vault = get_vault_pda();

    ctx.program()
        .accounts(client::accounts::CreateAdmin {
            admin: *admin,
            config: config_pda,
            stature_vault,
            system_program: SYSTEM_PROGRAM_ID,
        })
        .args(client::args::CreateAdmin {})
        .instruction()
        .unwrap()
}

pub fn tx_init_config(ctx: &mut AnchorContext) -> Keypair {
    let admin = ctx
        .svm
        .create_funded_account(LAMPORTS_PER_SOL * 10)
        .unwrap();
    let config_pda = get_config_pda();

    let ix = ix_init_config(ctx, &admin.pubkey());
    ctx.execute_instruction(ix, &[&admin])
        .unwrap()
        .assert_success();
    println!("config after init: {:?}", ctx.svm.get_account(&config_pda));

    admin
}

pub fn ix_register_program(
    ctx: &mut AnchorContext,
    payer: &Pubkey,
    target_program: &Pubkey,
    name: &String,
) -> Instruction {
    // 1. Derive the Stature-specific Registry PDA
    let registered_program_pda = get_registered_program_pda(target_program);
    // 2. Derive the system-level ProgramData PDA
    let program_data_pda = get_program_data_pda(target_program);

    ctx.program()
        .accounts(client::accounts::CreateProgram {
            authority: *payer,
            target_program: *target_program,
            program_data: program_data_pda,
            registered_program: registered_program_pda,
            system_program: SYSTEM_PROGRAM_ID,
        })
        .args(client::args::CreateProgram { name: name.clone() })
        .instruction()
        .unwrap()
}

pub fn tx_register_program(ctx: &mut AnchorContext, name: String, owner: &Keypair) -> Pubkey {
    let target_program = Pubkey::new_unique();

    mock_program_data(ctx, &target_program, &owner.pubkey());

    let ix_register = ix_register_program(ctx, &owner.pubkey(), &target_program, &name);
    ctx.execute_instruction(ix_register, &[&owner])
        .unwrap()
        .assert_success();

    target_program
}

pub fn tx_verified_program(
    ctx: &mut AnchorContext,
    admin: &Keypair,
    name: String,
    owner: &Keypair,
) -> Pubkey {
    let target_program = tx_register_program(ctx, name, owner);

    // 2. Register a program
    let ix_verify = ix_verify_registed_program(ctx, &admin.pubkey(), &target_program);
    ctx.execute_instruction(ix_verify, &[&admin])
        .unwrap()
        .assert_success();

    target_program
}

pub fn tx_registered_verified_statured_program(
    ctx: &mut AnchorContext,
    admin: &Keypair,
    program_owner: &Keypair,
) -> Pubkey {
    let program_name = "First Program".to_string();
    let target_program = tx_verified_program(ctx, &admin, program_name, &program_owner);

    let program_stature = 60 as i64;
    let ix_update_stature =
        ix_manage_program_stature(ctx, &admin.pubkey(), &target_program, program_stature);
    ctx.execute_instruction(ix_update_stature, &[&admin])
        .unwrap()
        .assert_success();

    target_program
}

pub fn ix_verify_registed_program(
    ctx: &mut AnchorContext,
    admin: &Pubkey,
    target_program: &Pubkey,
) -> Instruction {
    let config_pda = get_config_pda();
    // 1. Derive the Stature-specific Registry PDA
    let registered_program_pda = get_registered_program_pda(target_program);
    // 2. Derive the system-level ProgramData PDA

    ctx.program()
        .accounts(client::accounts::UpdateProgramVerifiedStatus {
            admin: *admin,
            target_program: *target_program,
            config: config_pda,
            registered_program: registered_program_pda,
        })
        .args(client::args::UpdateProgramVerifiedStatus {})
        .instruction()
        .unwrap()
}

pub fn ix_register_user(ctx: &mut AnchorContext, owner: &Pubkey, name: &String) -> Instruction {
    let user_pda = get_user_pda(owner);

    ctx.program()
        .accounts(client::accounts::CreateUser {
            owner: *owner,
            user: user_pda,
            system_program: SYSTEM_PROGRAM_ID,
        })
        .args(client::args::CreateUser { name: name.clone() })
        .instruction()
        .unwrap()
}

pub fn ix_init_program_user_state(
    ctx: &mut AnchorContext,
    target_program: &Pubkey,
    program_owner: &Keypair,
    user_wallet: &Pubkey,
) -> Instruction {
    let user_pda = get_user_pda(&user_wallet);
    let registered_program_pda = get_registered_program_pda(target_program);
    let program_user_state = get_program_user_state_pda(&registered_program_pda, &user_wallet);

    ctx.program()
        .accounts(client::accounts::CreateProgramUserState {
            authority: program_owner.pubkey(),
            user_wallet: *user_wallet,
            user: user_pda,
            target_program: *target_program,
            registered_program: registered_program_pda,
            system_program: SYSTEM_PROGRAM_ID,
            program_user_state: program_user_state
        })
        .args(client::args::CreateProgramUserState {})
        .instruction()
        .unwrap()
}

pub fn ix_update_stature_cpi(
    ctx: &mut AnchorContext,
    program_authority: &Pubkey,
    target_program: &Pubkey,
    user_wallet: &Pubkey,
    source_account: &Pubkey,
    val: i64,
    memo: String,
) -> Instruction {
    let registered_program_pda = get_registered_program_pda(target_program);
    let user_pda = get_user_pda(&user_wallet);
    let state_pda = get_program_user_state_pda(&registered_program_pda, &user_wallet);
    let record_pda = get_record_pda(
        &user_wallet,
        &registered_program_pda,
        source_account,
    );
    let vault_pda = get_vault_pda();

    ctx.program()
        .accounts(client::accounts::UpdateUserStature {
            signer: *program_authority,
            target_program: *target_program,
            registered_program: registered_program_pda,
            user_wallet: *user_wallet,
            user: user_pda,
            registered_program_source_account: *source_account,
            program_user_state: state_pda,
            record: record_pda,
            system_program: SYSTEM_PROGRAM_ID,
            stature_vault: vault_pda, 
        })
        .args(client::args::UpdateUserStature {
            tx_value_lamports: val,
            memo,
        })
        .instruction()
        .unwrap()
}
