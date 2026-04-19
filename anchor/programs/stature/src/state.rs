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
    pub total_records: u64,
    // pub db_id: u64,  // make it string beacuse its uuid
    pub bump: u8,
}

#[account]
#[derive(InitSpace, Debug)]
pub struct User {
    pub wallet: Pubkey,

    #[max_len(32)]
    pub name: String,

    pub stature: i64,
    pub is_suspended: bool,

    pub record_idx: u64,

    pub bump: u8,
}

#[account]
#[derive(InitSpace, Debug)]
pub struct RegisteredProgram {
    pub program_id: Pubkey,
    pub approved_by: Pubkey, // The Stature Admin who approved this program

    #[max_len(32)]
    pub name: String,

    pub stature: i64,
    pub total_positive_raw: i64,
    pub total_negative_raw: i64,

    pub max_record_cap: u64,
    pub record_count: u64,

    pub weight: u8,
    pub is_verified: bool,
    pub is_suspended: bool,
    pub bump: u8,
}
