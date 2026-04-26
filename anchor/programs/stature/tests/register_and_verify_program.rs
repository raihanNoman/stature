#![allow(unexpected_cfgs)]

use anchor_lang;
use anchor_litesvm::Signer;

use helpers::setup::*;
use litesvm_utils::TestHelpers;

mod helpers;
use crate::helpers::ix::{tx_init_config, tx_verified_program};

use crate::stature::accounts::RegisteredProgram;
use anchor_lang::AccountDeserialize;
anchor_lang::declare_program!(stature);

#[test]
fn test_registered_and_verified_program() {
    let mut ctx = setup_ctx();
    let admin = tx_init_config(&mut ctx);
    let program_owner_wallet = ctx.svm.create_funded_account(LAMPORTS_PER_SOL).unwrap();
    let program_name = "First Program".to_string();
    let target_program = tx_verified_program(&mut ctx, &admin, program_name, &program_owner_wallet);

    let program_pda = get_registered_program_pda(&target_program);
    let account = ctx
        .svm
        .get_account(&program_pda)
        .expect("registered program account should exist");

    // --- Verification Logic ---
    // Anchor adds an 8-byte discriminator to the start of the data
    let mut data: &[u8] = &account.data;
    let program_acc = RegisteredProgram::try_deserialize(&mut data)
        .expect("Should be able to deserialize RegisteredProgram");

    assert!(
        program_acc.is_verified,
        "is_verified should be true after admin approval"
    );
    assert_eq!(
        program_acc.approved_by,
        admin.pubkey(),
        "Admin pubkey mismatch"
    );

    // In litesvm, you can further deserialize 'account.data'
    // to check if name == "Alice" and stature == 0
    assert!(account.data.len() > 0);
}
