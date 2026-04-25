#![allow(unexpected_cfgs)]

use anchor_lang;
use anchor_litesvm::{AnchorContext, AnchorLiteSVM, Instruction, Pubkey, Signer};
//litesvm::solana_sdk::account::Account;

use litesvm_utils::TestHelpers;
use solana_sdk::account::Account;

use crate::stature::client;
// use crate::stature::cpi::Und
anchor_lang::declare_program!(stature);

const SYSTEM_PROGRAM_ID: Pubkey = anchor_lang::system_program::ID;
const BPF_LOADER_UPGRADEABLE_ID: Pubkey =
    solana_sdk::pubkey!("BPFLoaderUpgradeab1e11111111111111111111111");
const PROGRAM_ID: Pubkey = solana_sdk::pubkey!("9VFHpUQnHsG94AKzGfzf4mAeunxcQw8G9am6FfVEBVZb");
const LAMPORTS_PER_SOL: u64 = 1_000_000_000;

// Helper to create an initialized test context
fn setup_ctx() -> anchor_litesvm::AnchorContext {
    AnchorLiteSVM::build_with_program(
        PROGRAM_ID,
        include_bytes!("../../../target/deploy/stature.so"),
    )
}

fn get_program_data_pda(target_program: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(&[target_program.as_ref()], &BPF_LOADER_UPGRADEABLE_ID).0
}

// -------------------------------
// PDA HELPERS
// -------------------------------
fn get_config_pda() -> Pubkey {
    Pubkey::find_program_address(&[b"config"], &PROGRAM_ID).0
}
fn get_vault_pda() -> Pubkey {
    Pubkey::find_program_address(&[b"stature_vault"], &PROGRAM_ID).0
}
fn get_registered_program_pda(target_program: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(
        &[b"registered_program", target_program.as_ref()],
        &PROGRAM_ID,
    )
    .0
}
fn get_user_pda(wallet: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(&[b"user", wallet.as_ref()], &PROGRAM_ID).0
}
fn get_program_user_state_pda(program_pda: &Pubkey, user_pda: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(
        &[b"state", program_pda.as_ref(), user_pda.as_ref()],
        &PROGRAM_ID,
    )
    .0
}
fn get_record_pda(user_pda: &Pubkey, program_pda: &Pubkey, source: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(
        &[
            b"record",
            user_pda.as_ref(),
            program_pda.as_ref(),
            source.as_ref(),
        ],
        &PROGRAM_ID,
    )
    .0
}

fn get_account_discriminator(name: &str) -> [u8; 8] {
    let mut hasher = solana_sdk::hash::Hasher::default();
    hasher.hash(format!("account:{}", name).as_bytes());
    let mut dst = [0u8; 8];
    dst.copy_from_slice(&hasher.result().to_bytes()[..8]);
    dst
}

fn ix_init_config(ctx: &mut AnchorContext, admin: &Pubkey) -> Instruction {
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

fn ix_register_program(
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

fn ix_update_stature(
    ctx: &mut AnchorContext,
    program_authority: &dyn Signer, // The key that owns the target program
    target_program: &Pubkey,
    user_wallet: &Pubkey,
    source_account: &Pubkey,
    val: i64,
) -> Instruction {
    let registered_program_pda = get_registered_program_pda(target_program);
    let user_pda = get_user_pda(user_wallet);
    let state_pda = get_program_user_state_pda(&registered_program_pda, &user_pda);
    let record_pda = get_record_pda(&user_pda, &registered_program_pda, source_account);

    ctx.program()
        .accounts(client::accounts::UpdateUserStature {
            signer: program_authority.pubkey(),
            target_program: *target_program,
            registered_program: registered_program_pda,
            user_wallet: *user_wallet,
            user: user_pda,
            registered_program_source_account: *source_account,
            program_user_state: state_pda,
            record: record_pda,
            system_program: SYSTEM_PROGRAM_ID,
        })
        .args(client::args::UpdateUserStature {
            tx_value_lamports: val,
            memo: "Test update".to_string(),
        })
        .instruction()
        .unwrap()
}

fn ix_manage_program_suspension(
    ctx: &mut AnchorContext,
    admin: &Pubkey,
    target_program: &Pubkey,
    is_suspended: bool,
) -> Instruction {
    let config_pda = get_config_pda();
    let registered_program_pda = get_registered_program_pda(target_program);

    ctx.program()
        .accounts(client::accounts::UpdateProgramSuspension {
            admin: *admin,
            config: config_pda,
            target_program: *target_program,
            registered_program: registered_program_pda,
        })
        .args(client::args::UpdateProgramSuspension { is_suspended })
        .instruction()
        .unwrap()
}

fn ix_manage_admin(
    ctx: &mut AnchorContext,
    current_admin: &Pubkey,
    new_admin: &Pubkey,
) -> Instruction {
    let config_pda = get_config_pda();

    ctx.program()
        .accounts(client::accounts::UpdateAdmin {
            admin: *current_admin,
            config: config_pda,
        })
        .args(client::args::UpdateAdmin {
            new_admin: *new_admin,
        })
        .instruction()
        .unwrap()
}

fn ix_register_user(ctx: &mut AnchorContext, owner: &Pubkey, name: &String) -> Instruction {
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

fn mock_source_account(ctx: &mut AnchorContext, owner: &Pubkey) -> Pubkey {
    let source_pubkey = Pubkey::new_unique();
    let rent = ctx.svm.minimum_balance_for_rent_exemption(0);

    ctx.svm
        .set_account(
            source_pubkey,
            Account {
                lamports: rent,
                data: vec![],
                owner: *owner,
                executable: false,
                rent_epoch: 0,
            },
        )
        .expect("Failed to set mock account");

    source_pubkey
}

fn mock_program_data(ctx: &mut AnchorContext, target_program: &Pubkey, authority: &Pubkey) {
    let program_data_pda = get_program_data_pda(target_program);

    // ProgramData Account Layout:
    // [0..4]: Enum Variant (3 for ProgramData)
    // [4..12]: Slot (u64)
    // [12..13]: Upgrade Authority Option (1 for Some, 0 for None)
    // [13..45]: Upgrade Authority Pubkey
    let mut data = vec![0u8; 45];
    data[0] = 3; // Variant
    data[12] = 1; // Option::Some
    data[13..45].copy_from_slice(authority.as_ref());

    ctx.svm
        .set_account(
            program_data_pda,
            solana_sdk::account::Account {
                lamports: 1_000_000_000,
                data,
                owner: BPF_LOADER_UPGRADEABLE_ID,
                executable: false,
                rent_epoch: 0,
            },
        )
        .expect("Failed to mock ProgramData");
}


// Update this helper in your test file
fn mock_new_user(ctx: &mut AnchorContext, user_wallet: &Pubkey) {
    let user_pda = get_user_pda(user_wallet);
    
    #[derive(anchor_lang::AnchorSerialize)]
    struct StatureUser {
        discriminator: [u8; 8],
        owner: Pubkey,
        stature: i64,
        points: i64,
    }

    let data = StatureUser {
        discriminator: get_account_discriminator("user"),
        owner: *user_wallet,
        stature: 0,
        points: 0,
    };

    ctx.svm.set_account(
        user_pda,
        solana_sdk::account::Account {
            lamports: 100_000_000,
            data: anchor_lang::prelude::borsh::to_vec(&data).unwrap(),
            owner: PROGRAM_ID,
            executable: false,
            rent_epoch: 0,
        },
    ).unwrap();
}

fn mock_verify_program_by_admin(ctx: &mut AnchorContext, target_program: &Pubkey) {
    let reg_pda = get_registered_program_pda(target_program);
    let mut account = ctx
        .svm
        .get_account(&reg_pda)
        .expect("Program not registered");

    // Anchor discriminator for RegisteredProgram
    let mut hasher = solana_sdk::hash::Hasher::default();
    hasher.hash(b"account:RegisteredProgram");
    let disc = &hasher.result().to_bytes()[..8];

    // We manually construct the fields we care about.
    // The rest we can leave as zeros, as long as the length is right.
    let mut new_data = vec![0u8; account.data.len()];
    new_data[..8].copy_from_slice(disc);

    // target_program at offset 8
    new_data[8..40].copy_from_slice(target_program.as_ref());

    // Now, find the 'is_verified' bit.
    // Given your struct, it follows: target(32), auth(32), approved(32), name(36), i64(8), i64(8), i64(8), u64(8), u64(8), u16(2)
    // 8 + 32 + 32 + 32 + 36 + 8 + 8 + 8 + 8 + 8 + 2 = 182.
    // IF THIS FAILS, check if 'name' is really 36 bytes (4 prefix + 32 max).
    new_data[182] = 1;

    ctx.svm
        .set_account(
            reg_pda,
            solana_sdk::account::Account {
                data: new_data,
                ..account
            },
        )
        .unwrap();
}

// Helper to bridge the gap if you haven't defined it
fn register_test_program(
    ctx: &mut AnchorContext,
    payer: &anchor_litesvm::Keypair,
    target: &Pubkey,
) {
    mock_program_data(ctx, target, &payer.pubkey());

    let name = "TestProg".to_string();
    let ix = ix_register_program(ctx, &payer.pubkey(), target, &name);
    ctx.execute_instruction(ix, &[payer])
        .unwrap()
        .assert_success();
}

#[test]
fn test_register_user() {
    let mut ctx = setup_ctx();
    let user_wallet = ctx.svm.create_funded_account(1_000_000_000).unwrap();
    let user_name = "Alice".to_string();

    // 1. Create Instruction
    let ix = ix_register_user(&mut ctx, &user_wallet.pubkey(), &user_name);

    // 2. Execute
    ctx.execute_instruction(ix, &[&user_wallet])
        .unwrap()
        .assert_success();

    // 3. Verify Account Data
    let user_pda = get_user_pda(&user_wallet.pubkey());
    let account = ctx
        .svm
        .get_account(&user_pda)
        .expect("User account should exist");

    // In litesvm, you can further deserialize 'account.data'
    // to check if name == "Alice" and stature == 0
    assert!(account.data.len() > 0);
    println!("✅ User {} successfully registered!", user_name);
}

#[test]
fn test_init_config() {
    let mut ctx = setup_ctx();
    let admin = ctx.svm.create_funded_account(10_000_000_000).unwrap();
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
    let admin = ctx.svm.create_funded_account(10_000_000_000).unwrap();
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

#[test]
fn test_attack_memo_bloat() {
    let mut ctx = setup_ctx();
    let auth = ctx.svm.create_funded_account(LAMPORTS_PER_SOL).unwrap();
    let prog = Pubkey::new_unique();
    let user = Pubkey::new_unique();
    let source = Pubkey::new_unique(); // In reality, this needs to be owned by prog

    let huge_memo = "a".repeat(100); // Max is 64
    let ix = ix_update_stature_cpi(
        &mut ctx,
        &auth.pubkey(),
        &prog,
        &user,
        &source,
        100,
        huge_memo,
    );

    let result = ctx.execute_instruction(ix, &[&auth]).unwrap();
    assert!(!result.is_success(), "Should fail due to memo length");
}

#[test]
fn test_attack_rate_limit() {
    let mut ctx = setup_ctx();
    let auth = ctx.svm.create_funded_account(LAMPORTS_PER_SOL).unwrap();
    let user_wallet = ctx.svm.create_funded_account(LAMPORTS_PER_SOL).unwrap();
    let target_program = Pubkey::new_unique();

    let separate_payer = ctx.svm.create_funded_account(LAMPORTS_PER_SOL).unwrap();

    register_test_program(&mut ctx, &separate_payer, &target_program);

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
        "First".into(),
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

#[test]
fn test_attack_fake_source_owner() {
    let mut ctx = setup_ctx();
    let auth = ctx.svm.create_funded_account(LAMPORTS_PER_SOL).unwrap();
    let target_program = Pubkey::new_unique();
    let user = Pubkey::new_unique();

    let separate_payer = ctx.svm.create_funded_account(LAMPORTS_PER_SOL).unwrap();

    register_test_program(&mut ctx, &separate_payer, &target_program);

    // Create a source account owned by the SYSTEM PROGRAM (incorrect owner)
    let fake_source = mock_source_account(&mut ctx, &anchor_lang::system_program::ID);

    let ix = ix_update_stature_cpi(
        &mut ctx,
        &auth.pubkey(),
        &target_program,
        &user,
        &fake_source,
        100,
        "test".into(),
    );

    let result = ctx.execute_instruction(ix, &[&auth]).unwrap();

    // Should fail with your custom ErrorCode::InvalidSourceOwner
    assert!(!result.is_success());
    assert!(result
        .logs()
        .iter()
        .any(|l| l.contains("InvalidSourceOwner")));
}

//Test D: The "Rent Drain" Attack

#[test]
fn test_security_rent_exhaustion_protection() {
    let mut ctx = setup_ctx();

    // 1. Setup - Create an authority with enough for exactly ONE update cycle
    // Protocol Fee (0.001) + Rent for User, State, and Record accounts (~0.012 SOL total)
    // We give it 13,000,000 lamports (0.013 SOL)
    let starting_balance = 13_000_000;
    let authority = ctx.svm.create_funded_account(starting_balance).unwrap();

    // Separate payer to register the program so we don't drain the authority yet
    let separate_payer = ctx.svm.create_funded_account(LAMPORTS_PER_SOL).unwrap();
    let target_program = Pubkey::new_unique();

    register_test_program(&mut ctx, &separate_payer, &target_program);

    // 2. FIRST UPDATE (Should Succeed)
    let user_1 = Pubkey::new_unique();
    let source_1 = mock_source_account(&mut ctx, &target_program);

    let ix1 = ix_update_stature_cpi(
        &mut ctx,
        &authority.pubkey(),
        &target_program,
        &user_1,
        &source_1,
        100,
        "first_success".into(),
    );

    let result1 = ctx.execute_instruction(ix1, &[&authority]).unwrap();
    result1.assert_success();

    let balance_after_one = ctx.svm.get_account(&authority.pubkey()).unwrap().lamports;
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
        &authority.pubkey(),
        &target_program,
        &user_2,
        &source_2,
        100,
        "second_failure".into(),
    );

    let result2 = ctx.execute_instruction(ix2, &[&authority]).unwrap();

    // 4. Assertions
    assert!(
        !result2.is_success(),
        "Transaction should have failed due to insufficient funds for rent/fees"
    );

    // Verify it failed for the right reason (check logs for 'InsufficientFunds')
    let logs_joined = result2.logs().join(" ");
    assert!(
        logs_joined.contains("insufficient lamports") || logs_joined.contains("InsufficientFunds"),
        "Expected insufficient funds error, but got: {:?}",
        result2.logs()
    );

    println!("✅ Security check passed: Authority cannot create more stature records than it can pay for.");
}

fn ix_update_stature_cpi(
    ctx: &mut AnchorContext,
    program_authority: &Pubkey,
    target_program: &Pubkey,
    user_wallet: &Pubkey,
    source_account: &Pubkey,
    val: i64,
    memo: String,
) -> Instruction {
    let registered_program_pda = get_registered_program_pda(target_program);
    let user_pda = get_user_pda(user_wallet);
    let state_pda = get_program_user_state_pda(&registered_program_pda, user_wallet);
    let record_pda = get_record_pda(user_wallet, &registered_program_pda, source_account);

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
        })
        .args(client::args::UpdateUserStature {
            tx_value_lamports: val,
            memo,
        })
        .instruction()
        .unwrap()
}

#[test]
fn test_manual_cpi_router_success() {
    let mut ctx = setup_ctx();

    // 1. Identities
    let auth = ctx.svm.create_funded_account(LAMPORTS_PER_SOL).unwrap();
    let target_prog = Pubkey::new_unique();
    let user_wallet = ctx.svm.create_funded_account(LAMPORTS_PER_SOL).unwrap();

    let separate_payer = ctx.svm.create_funded_account(LAMPORTS_PER_SOL).unwrap();

    register_test_program(&mut ctx, &separate_payer, &target_prog); // ADD THIS LINE
                                                                    // 2. Mock Admin Verification (fixes 6006 error)
    mock_verify_program_by_admin(&mut ctx, &target_prog);
    // 3. Mock User Initialization (fixes 3012 error)
    mock_new_user(&mut ctx, &user_wallet.pubkey());
    // 4. Mock Source Account

    // 2. Mock Accounts
    let source = mock_source_account(&mut ctx, &target_prog);

    // 3. Derive PDAs
    let reg_prog_pda = get_registered_program_pda(&target_prog);
    let user_pda = get_user_pda(&user_wallet.pubkey());
    let state_pda = get_program_user_state_pda(&reg_prog_pda, &user_wallet.pubkey());
    let record_pda = get_record_pda(&user_wallet.pubkey(), &reg_prog_pda, &source);
    let vault_pda = get_vault_pda();

    // 4. Calculate Discriminator for "invoke_stature_update_cpi"
    // Anchor formula: sha256("global:invoke_stature_update_cpi")[..8]
    let mut hasher = solana_sdk::hash::Hasher::default();
    hasher.hash(b"global:invoke_stature_update_cpi");
    let discriminator = &hasher.result().to_bytes()[..8];

    // 5. Build Instruction Data (Discriminator + tx_value + memo)

    // We use a simple struct to Borsh-serialize the args
    #[derive(anchor_lang::AnchorSerialize)]
    struct LocalArgs {
        tx_value: i64,
        memo: String,
    }

    let args = LocalArgs {
        tx_value: 5000,
        memo: "Manual Router Test".to_string(),
    };

    let mut ix_data = discriminator.to_vec();
    // Explicitly use the trait method
    let args_bytes = anchor_lang::prelude::borsh::to_vec(&args).expect("Failed to serialize args");
    ix_data.extend(args_bytes);
    // 6. Build Account Metas
    // Order must match your StatureUpdateBundle struct exactly!
    let accounts = vec![
        solana_sdk::instruction::AccountMeta::new(auth.pubkey(), true),
        solana_sdk::instruction::AccountMeta::new_readonly(target_prog, false),
        solana_sdk::instruction::AccountMeta::new(reg_prog_pda, false),
        solana_sdk::instruction::AccountMeta::new_readonly(user_wallet.pubkey(), false),
        solana_sdk::instruction::AccountMeta::new(user_pda, false),
        solana_sdk::instruction::AccountMeta::new_readonly(source, false),
        solana_sdk::instruction::AccountMeta::new(state_pda, false),
        solana_sdk::instruction::AccountMeta::new(record_pda, false),
        solana_sdk::instruction::AccountMeta::new(vault_pda, false),
        solana_sdk::instruction::AccountMeta::new_readonly(PROGRAM_ID, false),
        solana_sdk::instruction::AccountMeta::new_readonly(SYSTEM_PROGRAM_ID, false),
    ];

    // 7. Create and Execute Instruction
    let ix = solana_sdk::instruction::Instruction {
        program_id: PROGRAM_ID,
        accounts,
        data: ix_data,
    };

    let result = ctx.execute_instruction(ix, &[&auth]).unwrap();

    // 8. Log and Assert
    if !result.is_success() {
        for log in result.logs() {
            println!("{}", log);
        }
    }
    assert!(result.is_success(), "The router call failed!");

    // 9. Verify the state was actually written
    let account = ctx
        .svm
        .get_account(&record_pda)
        .expect("Record PDA not found");
    assert!(account.lamports > 0, "Record account should have balance");
    println!("✅ Manual Router Test Success: Stature updated via lib.rs routing!");
}
