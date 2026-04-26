#![allow(unexpected_cfgs)]

use anchor_lang::{self, prelude::pubkey};
use anchor_litesvm::{AnchorLiteSVM, Pubkey};
use litesvm_utils::{Keypair, Signer};

pub const SYSTEM_PROGRAM_ID: Pubkey = anchor_lang::system_program::ID;
pub const BPF_LOADER_UPGRADEABLE_ID: Pubkey =
    solana_sdk::pubkey!("BPFLoaderUpgradeab1e11111111111111111111111");
pub const PROGRAM_ID: Pubkey = solana_sdk::pubkey!("9VFHpUQnHsG94AKzGfzf4mAeunxcQw8G9am6FfVEBVZb");
pub const LAMPORTS_PER_SOL: u64 = 1_000_000_000;

// Helper to create an initialized test context
// -------------------------------
// PDA HELPERS
// -------------------------------




anchor_lang::declare_program!(stature);

pub fn setup_ctx() -> anchor_litesvm::AnchorContext {
    AnchorLiteSVM::build_with_program(
        self::stature::ID,
        include_bytes!("../../../../target/deploy/stature.so"),
    )
}

pub fn get_program_data_pda(target_program: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(&[target_program.as_ref()], &BPF_LOADER_UPGRADEABLE_ID).0
}

// -------------------------------
// PDA HELPERS
// -------------------------------
pub fn get_config_pda() -> Pubkey {
    Pubkey::find_program_address(&[b"config"], &PROGRAM_ID).0
}
pub fn get_vault_pda() -> Pubkey {
    Pubkey::find_program_address(&[b"stature_vault"], &PROGRAM_ID).0
}
pub fn get_registered_program_pda(target_program: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(
        &[b"registered_program", target_program.as_ref()],
        &PROGRAM_ID,
    )
    .0
}
pub fn get_user_pda(wallet: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(&[b"user", wallet.as_ref()], &PROGRAM_ID).0
}
pub fn get_program_user_state_pda(program_pda: &Pubkey, user_wallet: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(
        &[b"state", program_pda.as_ref(), user_wallet.as_ref()],
        &PROGRAM_ID,
    )
    .0
}
pub fn get_record_pda(user_pda: &Pubkey, program_pda: &Pubkey, source: &Pubkey) -> Pubkey {
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

pub fn get_account_discriminator(name: &str) -> [u8; 8] {
    let mut hasher = solana_sdk::hash::Hasher::default();
    hasher.hash(format!("account:{}", name).as_bytes());
    let mut dst = [0u8; 8];
    dst.copy_from_slice(&hasher.result().to_bytes()[..8]);
    dst
}
