#![allow(unexpected_cfgs)]

use anchor_lang::{self};
use anchor_litesvm::{Pubkey, Signer};
//litesvm::solana_sdk::account::Account;

use anchor_lang::solana_program::clock::Clock;
use litesvm_utils::TestHelpers;

mod helpers;
use crate::helpers::ix::{
    ix_update_stature_cpi, tx_init_config, tx_registered_verified_statured_program,
};
use crate::helpers::mock::mock_source_account;
use crate::helpers::setup::{setup_ctx, LAMPORTS_PER_SOL, SYSTEM_PROGRAM_ID};

// use crate::stature::cpi::Und
anchor_lang::declare_program!(stature);

//We proved that your program cannot be used as a "money printer" or a "rent drain" on the program owner’s wallet
//so beyond what they can actually afford.

#[test]
fn test_security_rent_exhaustion_protection() {
    let mut ctx = setup_ctx();
    let admin = tx_init_config(&mut ctx);

    let program_owner = ctx.svm.create_funded_account(LAMPORTS_PER_SOL).unwrap();
    let target_program = tx_registered_verified_statured_program(&mut ctx, &admin, &program_owner);

    // let program_owner = ctx.svm.create_funded_account(LAMPORTS_PER_SOL).unwrap();

    // 1. Setup - Create an authority with enough for exactly ONE update cycle
    // Protocol Fee (0.001) + Rent for User, State, and Record accounts (~0.012 SOL total)
    // We give it 13,000,000 lamports (0.013 SOL)
    let starting_balance = 8_000_000;

    ctx.svm
        .set_account(
            program_owner.pubkey(),
            solana_sdk::account::Account {
                lamports: starting_balance,
                data: vec![],
                owner: SYSTEM_PROGRAM_ID,
                executable: false,
                rent_epoch: 0,
            },
        )
        .expect("Failed to reduce balance");

    // 2. FIRST UPDATE (Should Succeed)
    let user_1 = Pubkey::new_unique();
    let source_1 = mock_source_account(&mut ctx, &target_program);

    let ix1 = ix_update_stature_cpi(
        &mut ctx,
        &program_owner.pubkey(),
        &target_program,
        &user_1,
        &source_1,
        100,
        "first_success".into(),
    );

    let result1 = ctx.execute_instruction(ix1, &[&program_owner]).unwrap();
    result1.assert_success();

    let clock = Clock {
        slot: 1000, // Increased slot
        epoch_start_timestamp: 0,
        epoch: 1,
        leader_schedule_epoch: 1,
        unix_timestamp: 1736640000 + (4 * 3600), // Current-ish time + 4 hours
    };

    ctx.svm.set_sysvar(&clock);

    println!("⏰ Time warped 3 hours into the future...");

    let balance_after_one = ctx
        .svm
        .get_account(&program_owner.pubkey())
        .unwrap()
        .lamports;
    println!(
        "💰 Balance after one update: {} lamports",
        balance_after_one
    );
    assert!(
        balance_after_one < starting_balance,
        "SOL should have been spent on rent/fees"
    );

    // 3. SECOND UPDATE (Should FAIL)
    // We try to initialize a second user's stature.
    // The authority has < 0.001 SOL left, which is not enough for the fee + new PDA rent.
    let user_2 = Pubkey::new_unique();
    let source_2 = mock_source_account(&mut ctx, &target_program);

    let ix2 = ix_update_stature_cpi(
        &mut ctx,
        &program_owner.pubkey(),
        &target_program,
        &user_2,
        &source_2,
        100,
        "second_failure".into(),
    );

    let result2 = ctx.execute_instruction(ix2, &[&program_owner]).unwrap();

    // 5. Assertions
    assert!(
        !result2.is_success(),
        "Should fail due to insufficient funds for the second user's rent"
    );
    println!("✅ Security check passed: Rent exhaustion protection confirmed.");
}
