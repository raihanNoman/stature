#![allow(unexpected_cfgs)]

use anchor_lang::{self};
use anchor_litesvm::{AnchorContext, Instruction, Pubkey, Signer};
//litesvm::solana_sdk::account::Account;

use litesvm_utils::{Keypair, TestHelpers};

use crate::stature::client;

mod helpers;
use crate::helpers::ix::{
    ix_init_program_user_state, ix_register_user, tx_init_config,
    tx_registered_verified_statured_program,
};
use crate::helpers::mock::mock_source_account;
use crate::helpers::setup::{
    get_program_user_state_pda, get_record_pda, get_registered_program_pda, get_user_pda,
    get_vault_pda, setup_ctx, LAMPORTS_PER_SOL, SYSTEM_PROGRAM_ID,
};

// use crate::stature::cpi::Und
anchor_lang::declare_program!(stature);

fn ix_cpi(
    ctx: &mut AnchorContext,
    program_authority: &dyn Signer, // The key that owns the target program
    target_program: &Pubkey,
    user_wallet: &Keypair,
    source_account: &Pubkey,
    tx_value: i64,
) -> Instruction {
    let registered_program_pda = get_registered_program_pda(target_program);
    let user_pda = get_user_pda(&user_wallet.pubkey());
    let state_pda = get_program_user_state_pda(&registered_program_pda, &user_wallet.pubkey());
    let record_pda = get_record_pda(
        &user_wallet.pubkey(),
        &registered_program_pda,
        source_account,
    );
    let vault_pda = get_vault_pda();

    ctx.program()
        .accounts(client::accounts::UpdateUserStature {
            signer: program_authority.pubkey(),
            target_program: *target_program,
            registered_program: registered_program_pda,
            user_wallet: user_wallet.pubkey(),
            user: user_pda,
            registered_program_source_account: *source_account,
            program_user_state: state_pda,
            record: record_pda,
            stature_vault: vault_pda,
            // stature_program: PROGRAM_ID,
            system_program: SYSTEM_PROGRAM_ID,
        })
        .args(client::args::UpdateUserStature {
            tx_value_lamports: tx_value,
            memo: "Test update".to_string(),
        })
        .instruction()
        .unwrap()
}

//Test D: The "Rent Drain" Attack
#[test]
fn test_update_stature_via_cpi() {
    let mut ctx = setup_ctx();
    let admin = tx_init_config(&mut ctx);

    let program_owner = ctx.svm.create_funded_account(LAMPORTS_PER_SOL).unwrap();
    let target_program = tx_registered_verified_statured_program(&mut ctx, &admin, &program_owner);
    let user_wallet = ctx.svm.create_funded_account(LAMPORTS_PER_SOL).unwrap();
    let source_account = mock_source_account(&mut ctx, &target_program);

    let user_pda = get_user_pda(&user_wallet.pubkey());
    if ctx.svm.get_account(&user_pda).is_none() {
        let ix_user = ix_register_user(&mut ctx, &user_wallet.pubkey(), &"User1".to_string());
        ctx.execute_instruction(ix_user, &[&user_wallet])
            .unwrap()
            .assert_success();
        println!("🆕 User registered because it didn't exist.");
    }

    // 2. Check for Program User State
    let registered_pda = get_registered_program_pda(&target_program);
    let state_pda = get_program_user_state_pda(&registered_pda, &user_wallet.pubkey());
    if ctx.svm.get_account(&state_pda).is_none() {
        // Run your manual init instruction here
        let ix_state = ix_init_program_user_state(
            &mut ctx,
            &target_program,
            &program_owner,
            &user_wallet.pubkey(),
        );
        ctx.execute_instruction(ix_state, &[&program_owner])
            .unwrap()
            .assert_success();
        println!("🆕 State PDA initialized because it didn't exist.");
    }

    let record_pda = get_record_pda(&user_wallet.pubkey(), &registered_pda, &source_account);
    println!("--- PDA Check ---");
    println!("Target Program: {}", target_program);
    println!("Registered PDA: {}", registered_pda);
    println!("State PDA:      {}", state_pda);
    println!("Record PDA:     {}", record_pda);
    println!("-----------------");

    let ix = ix_cpi(
        &mut ctx,
        &program_owner,
        &target_program,
        &user_wallet,
        &source_account,
        400,
    );

    ctx.execute_instruction(ix, &[&program_owner])
        .unwrap()
        .assert_success();

    // // 2. Mock Accounts

    // // 3. Derive PDAs
    // let user_pda = get_user_pda(&user_wallet.pubkey());
    // let state_pda = get_program_user_state_pda(&target_program, &user_wallet.pubkey());
    // let record_pda = get_record_pda(&user_wallet.pubkey(), &target_program, &source);
    // let vault_pda = get_vault_pda();

    // // 4. Calculate Discriminator for "invoke_stature_update_cpi"
    // // Anchor formula: sha256("global:invoke_stature_update_cpi")[..8]
    // let mut hasher = solana_sdk::hash::Hasher::default();
    // hasher.hash(b"global:invoke_stature_update_cpi");
    // let discriminator = &hasher.result().to_bytes()[..8];

    // // 5. Build Instruction Data (Discriminator + tx_value + memo)

    // // We use a simple struct to Borsh-serialize the args
    // #[derive(anchor_lang::AnchorSerialize)]
    // struct LocalArgs {
    //     tx_value: i64,
    //     memo: String,
    // }

    // let args = LocalArgs {
    //     tx_value: 5000,
    //     memo: "Manual Router Test".to_string(),
    // };

    // let mut ix_data = discriminator.to_vec();
    // // Explicitly use the trait method
    // let args_bytes = anchor_lang::prelude::borsh::to_vec(&args).expect("Failed to serialize args");
    // ix_data.extend(args_bytes);
    // // 6. Build Account Metas
    // // Order must match your StatureUpdateBundle struct exactly!
    // let accounts = vec![
    //     solana_sdk::instruction::AccountMeta::new(admin.pubkey(), true),
    //     solana_sdk::instruction::AccountMeta::new_readonly(target_program, false),
    //     solana_sdk::instruction::AccountMeta::new(target_program, false),
    //     solana_sdk::instruction::AccountMeta::new_readonly(user_wallet.pubkey(), false),
    //     solana_sdk::instruction::AccountMeta::new(user_pda, false),
    //     solana_sdk::instruction::AccountMeta::new_readonly(source, false),
    //     solana_sdk::instruction::AccountMeta::new(state_pda, false),
    //     solana_sdk::instruction::AccountMeta::new(record_pda, false),
    //     solana_sdk::instruction::AccountMeta::new(vault_pda, false),
    //     solana_sdk::instruction::AccountMeta::new_readonly(PROGRAM_ID, false),
    //     solana_sdk::instruction::AccountMeta::new_readonly(SYSTEM_PROGRAM_ID, false),
    // ];

    // // 7. Create and Execute Instruction
    // let ix = solana_sdk::instruction::Instruction {
    //     program_id: PROGRAM_ID,
    //     accounts,
    //     data: ix_data,
    // };

    // let result = ctx.execute_instruction(ix, &[&admin]).unwrap();

    // // 8. Log and Assert
    // if !result.is_success() {
    //     for log in result.logs() {
    //         println!("{}", log);
    //     }
    // }
    // assert!(result.is_success(), "The router call failed!");

    // // 9. Verify the state was actually written
    // let account = ctx
    //     .svm
    //     .get_account(&record_pda)
    //     .expect("Record PDA not found");
    // assert!(account.lamports > 0, "Record account should have balance");
    // println!("✅ Manual Router Test Success: Stature updated via lib.rs routing!");
}
