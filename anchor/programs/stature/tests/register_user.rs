#![allow(unexpected_cfgs)]

use anchor_lang;
use anchor_litesvm::Signer;

use helpers::setup::*;
use litesvm_utils::TestHelpers;

mod helpers;
use crate::helpers::ix::ix_register_user;

anchor_lang::declare_program!(stature);

#[test]
fn test_register_user() {
    let mut ctx = setup_ctx();
    let user_wallet = ctx.svm.create_funded_account(LAMPORTS_PER_SOL).unwrap();
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
