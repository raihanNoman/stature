#![allow(unexpected_cfgs)]
use anchor_lang;
mod helpers;
use anchor_litesvm::{Pubkey, Signer};
use helpers::setup::*;
use litesvm_utils::TestHelpers;

use crate::helpers::ix::ix_update_stature_cpi;
use crate::helpers::mock::{mock_source_account, register_test_program};

anchor_lang::declare_program!(stature);

#[test]
fn test_attack_rate_limit() {
    let mut ctx = setup_ctx();
    let auth = ctx
        .svm
        .create_funded_account(LAMPORTS_PER_SOL * 10)
        .unwrap();
    let user_wallet = ctx.svm.create_funded_account(LAMPORTS_PER_SOL).unwrap();
    let target_program = Pubkey::new_unique();
    let program_owner = ctx.svm.create_funded_account(LAMPORTS_PER_SOL).unwrap();

    register_test_program(&mut ctx, &program_owner, &target_program);

    // 1. Properly mock the source account using the helper
    let source = mock_source_account(&mut ctx, &target_program);

    // 2. First update (Success)
    let ix1 = ix_update_stature_cpi(
        &mut ctx,
        &auth.pubkey(),
        &target_program,
        &user_wallet.pubkey(),
        &source,
        100,
        "Memo for updating stature: Could have uuid for dynamo db".into(),
    );
    let _ = ctx.execute_instruction(ix1, &[&auth]);

    // 3. Second update immediately after (Should Fail)
    let ix2 = ix_update_stature_cpi(
        &mut ctx,
        &auth.pubkey(),
        &target_program,
        &user_wallet.pubkey(),
        &source,
        100,
        "Second".into(),
    );
    let result = ctx.execute_instruction(ix2, &[&auth]).unwrap();

    // In litesvm, failures are often returned as TransactionErrors
    assert!(!result.is_success(), "Rate limit should have triggered");
}
