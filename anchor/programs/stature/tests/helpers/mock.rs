#![allow(unexpected_cfgs)]
use anchor_lang;

use anchor_litesvm::{AnchorContext, Pubkey, Signer};

use crate::helpers::{
    ix::ix_register_program,
    setup::{get_program_data_pda, BPF_LOADER_UPGRADEABLE_ID},
};

anchor_lang::declare_program!(stature);

pub fn mock_program_data(ctx: &mut AnchorContext, target_program: &Pubkey, authority: &Pubkey) {
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

// Helper to bridge the gap if you haven't defined it
pub fn register_test_program(
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

pub fn mock_source_account(ctx: &mut AnchorContext, owner: &Pubkey) -> Pubkey {
    let source_pubkey = Pubkey::new_unique();
    let rent = ctx.svm.minimum_balance_for_rent_exemption(0);

    ctx.svm
        .set_account(
            source_pubkey,
            solana_sdk::account::Account {
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


