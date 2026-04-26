#![allow(unexpected_cfgs)]

use anchor_lang;
use anchor_litesvm::Signer;

use helpers::setup::*;
use litesvm_utils::TestHelpers;

mod helpers;
use crate::helpers::ix::{
    ix_init_program_user_state, ix_register_user, tx_init_config,
    tx_registered_verified_statured_program,
};
use crate::stature::accounts::ProgramUserState;
use anchor_lang::AccountDeserialize;
anchor_lang::declare_program!(stature);

#[test]
fn test_init_program_state() {
    let mut ctx = setup_ctx();
    let admin = tx_init_config(&mut ctx);
    let program_owner_wallet = ctx.svm.create_funded_account(LAMPORTS_PER_SOL).unwrap();
    let target_program =
        tx_registered_verified_statured_program(&mut ctx, &admin, &program_owner_wallet);

    let user_wallet = ctx.svm.create_funded_account(LAMPORTS_PER_SOL).unwrap();

    let user_pda = get_user_pda(&user_wallet.pubkey());
    if ctx.svm.get_account(&user_pda).is_none() {
        let ix_user = ix_register_user(&mut ctx, &user_wallet.pubkey(), &"User1".to_string());
        ctx.execute_instruction(ix_user, &[&user_wallet])
            .unwrap()
            .assert_success();
        println!("🆕 User registered because it didn't exist.");
    }

    let ix_state = ix_init_program_user_state(
        &mut ctx,
        &target_program,
        &program_owner_wallet,
        &user_wallet.pubkey(),
    );
    ctx.execute_instruction(ix_state, &[&program_owner_wallet])
        .unwrap()
        .assert_success();
    println!("🆕 State PDA initialized because it didn't exist.");

    let registered_pda = get_registered_program_pda(&target_program);
    let state_pda = get_program_user_state_pda(&registered_pda, &user_wallet.pubkey());

    let account = ctx
        .svm
        .get_account(&state_pda)
        .expect("ProgramUserState account should exist");

    let mut data: &[u8] = &account.data;
    let state_acc = ProgramUserState::try_deserialize(&mut data)
        .expect("Should be able to deserialize ProgramUserState");

    // Verify the fields you set in your 'init_program_user_state' function
    //  assert!(state_acc.first_action_at > 0, "Timestamp should be set");
    assert_eq!(state_acc.total_records, 0, "Initial records should be zero");

    // Verify the PDA bump was stored correctly (best practice)
    assert!(state_acc.bump > 0);
    println!("✅ Verification complete: ProgramUserState is initialized and correct.");
}
