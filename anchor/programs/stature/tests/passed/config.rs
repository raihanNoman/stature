#![allow(unexpected_cfgs)]

use anchor_lang;
use anchor_litesvm::Signer;
mod helpers;
use helpers::setup::*;
use litesvm_utils::TestHelpers;

use crate::helpers::ix::ix_init_config;

anchor_lang::declare_program!(stature);

#[test]
fn test_init_config() {
    let mut ctx = setup_ctx();
    let admin = ctx
        .svm
        .create_funded_account(LAMPORTS_PER_SOL * 10)
        .unwrap();
    let config_pda = get_config_pda();

    let ix = ix_init_config(&mut ctx, &admin.pubkey());
    ctx.execute_instruction(ix, &[&admin])
        .unwrap()
        .assert_success();

    let acc = ctx.svm.get_account(&config_pda);
    assert!(acc.is_some());
}

#[test]
fn test_prevent_double_init_config() {
    let mut ctx = setup_ctx();
    let admin = ctx
        .svm
        .create_funded_account(LAMPORTS_PER_SOL * 10)
        .unwrap();
    let config_pda = get_config_pda();

    // 1. FIRST INIT (should succeed)
    let ix = ix_init_config(&mut ctx, &admin.pubkey());
    ctx.execute_instruction(ix, &[&admin])
        .unwrap()
        .assert_success();
    println!("config after init: {:?}", ctx.svm.get_account(&config_pda));
    assert!(ctx.svm.get_account(&config_pda).is_some()); // Ensure account exists

    // 2. SECOND INIT (should FAIL)
    let ix2 = ix_init_config(&mut ctx, &admin.pubkey());
    let result = ctx.execute_instruction(ix2, &[&admin]).unwrap();

    assert!(
        !result.is_success(),
        "Second init should fail but succeeded: {:?}",
        result
    );
}
