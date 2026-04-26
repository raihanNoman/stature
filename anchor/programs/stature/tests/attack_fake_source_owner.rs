#![allow(unexpected_cfgs)]

use anchor_lang::{self};
use anchor_litesvm::{Pubkey, Signer};
//litesvm::solana_sdk::account::Account;

use litesvm_utils::TestHelpers;

mod helpers;
use crate::helpers::ix::{ix_update_stature_cpi, tx_init_config, tx_verified_program};
use crate::helpers::ix_management::ix_manage_program_stature;
use crate::helpers::mock::mock_source_account;
use crate::helpers::setup::{setup_ctx, LAMPORTS_PER_SOL};

// use crate::stature::cpi::Und
anchor_lang::declare_program!(stature);

#[test]
fn test_attack_fake_source_owner() {
    let mut ctx = setup_ctx();
    let admin = tx_init_config(&mut ctx);
    let program_name = "First Program".to_string();
    let program_owner = ctx.svm.create_funded_account(LAMPORTS_PER_SOL).unwrap();
    let target_program = tx_verified_program(&mut ctx, &admin, program_name, &program_owner);

    let program_stature = 60 as i64;
    let ix_update_stature =
        ix_manage_program_stature(&mut ctx, &admin.pubkey(), &target_program, program_stature);
    ctx.execute_instruction(ix_update_stature, &[&admin])
        .unwrap()
        .assert_success();

    // or more simply
    // let program_owner = ctx.svm.create_funded_account(LAMPORTS_PER_SOL).unwrap();
    // let target_program = tx_registered_verified_statured_program(&mut ctx, &admin, &program_owner);

    let user = Pubkey::new_unique();
    let fake_program = Pubkey::new_unique();

    // Create a source account owned by the fake program(incorrect owner)
    let fake_source = mock_source_account(&mut ctx, &fake_program);

    let ix = ix_update_stature_cpi(
        &mut ctx,
        &program_owner.pubkey(),
        &target_program,
        &user,
        &fake_source,
        100,
        "test".into(),
    );

    let result = ctx.execute_instruction(ix, &[&program_owner]).unwrap();

    for log in result.logs() {
        println!("{}", log);
    }

    // Should fail with your custom ErrorCode::InvalidSourceOwner
    assert!(!result.is_success());

    let err = result.error().unwrap();
    println!("Error: {:?}", err);

    assert!(result
        .logs()
        .iter()
        .any(|l| l.contains("InvalidSourceOwner")));
}

//Test D: The "Rent Drain" Attack
