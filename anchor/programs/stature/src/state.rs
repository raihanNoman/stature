use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace, Debug)]
pub struct Config {
    pub admin: Pubkey,
    pub bump: u8,
    pub is_initilized: bool
}


#[account]
#[derive(InitSpace, Debug)]
pub struct StatureRecord {
    pub company: Pubkey,
    pub user: Pubkey,

    pub amount: i64,
    pub timestamp: i64,
    
    pub user_record_idx : u64, 
    pub bump: u8,
}

#[account]
#[derive(InitSpace, Debug)]
pub struct CompanyUserState {
    pub last_updated_at: i64,
    pub last_nonce: u64, 
    // pub db_id: u64,  // make it string beacuse its uuid
    pub bump: u8,
}


#[account]
#[derive(InitSpace, Debug)]
pub struct User {
    pub id: Pubkey,

    #[max_len(32)]
    pub name: String,

    pub stature: i64,
    pub is_suspended: bool, 

    pub record_idx: u64, 

    pub bump: u8,
}




#[account]
#[derive(InitSpace, Debug)]
pub struct Company {
    pub admin: Pubkey,        // who approved it
    pub company_rep: Pubkey,  // who operates it

    #[max_len(32)]
    pub name: String,

    pub stature : i64, // controls company's reputation - allows me to slash reputation if company makes a mistake
   
    pub max_record_cap: u64, 
    pub record_count: u64,
    pub total_positive_raw: i64,
    pub total_negative_raw: i64,

    pub is_verified: bool, 
    pub is_suspended: bool, 

    pub weight: u8, // 10 = 1x, 20 = 2x, etc.

    pub bump: u8,
}

