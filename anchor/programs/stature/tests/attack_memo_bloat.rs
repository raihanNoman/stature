#![allow(unexpected_cfgs)]

use anchor_lang;
use anchor_litesvm::{Pubkey, Signer};
mod helpers;
use helpers::setup::*;
use litesvm_utils::TestHelpers;

use crate::helpers::ix::ix_update_stature_cpi;

anchor_lang::declare_program!(stature);

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
