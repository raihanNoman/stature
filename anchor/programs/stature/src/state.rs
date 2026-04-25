use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace, Debug)]
pub struct Config {
    pub admin: Pubkey,
    pub bump: u8,
    pub is_initilized: bool,
}

#[account]
#[derive(InitSpace, Debug)]
pub struct StatureRecord {
    pub registered_program: Pubkey,
    pub registered_program_source_account: Pubkey,
    pub user: Pubkey,

    pub stature: i64,
    pub tx_value: i64, // <--- Add this (The actual Lamports/Tokens)
    pub weight: u16,
    pub timestamp: i64,

    #[max_len(64)]
    pub memo: String, // reason for this record // or a uuid like connection to data-base

    pub user_record_idx: u64,
    pub bump: u8,
}

#[account]
#[derive(InitSpace, Debug)]
pub struct ProgramUserState {
    pub last_updated_at: i64,
    pub first_action_at: i64,
    pub total_records: u64,
    // pub db_id: u64,  // make it string beacuse its uuid
    pub bump: u8,
}

#[account]
#[derive(InitSpace, Debug)]
pub struct StatureUser {
    pub wallet: Pubkey, // 32
    #[max_len(32)]
    pub name: String, // 4 + 32
    pub stature: i64,   // 8
    pub is_suspended: bool, // 1
    pub total_records: u64, // 8
    pub first_action_at: i64, // 8

    // User Default Rate.
    pub total_positive_tx: u64, //Total value the user has successfully "honored" or contributed.
    pub total_negative_tx: u64, // Total value the user has "failed" on (e.g., missed payments, slashed amounts).

    pub since: i64, // 8
    pub bump: u8,   // 1
}

#[account]
#[derive(InitSpace, Debug)]
pub struct RegisteredProgram { 
    pub target_program: Pubkey,
    pub authority: Pubkey,

    pub approved_by: Pubkey, // The Stature Admin who approved this program

    #[max_len(32)]
    pub name: String,

    // Tells you if a program is trusted.
    pub stature: i64,

    // Tell you if a program is big.  //  Risk Ratio = total_negative_tx / total_positive_tx
    pub total_positive_tx: i64, // transaction amount in lanports
    pub total_negative_tx: i64,

    pub max_record_cap: u64,
    pub record_count: u64,

    pub weight: u16,
    pub is_verified: bool,
    pub is_suspended: bool,

    pub since: i64, // cread at
    pub bump: u8,
}
