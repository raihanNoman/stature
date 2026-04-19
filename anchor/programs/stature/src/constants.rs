use anchor_lang::prelude::*;

#[constant]
// // https://pyth.network/developers/price-feed-ids#solana-stable
// pub const SOL_USD_FEED_ID: &str = "0xef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d";
// pub const USDC_USD_FEED_ID: &str = "0xeaa020c61cc479712813461ce153894a96a6c00b21ed0cfc2798d1f9a9e9c94a";
// pub const MAXIMUM_AGE: u64 = 100; // allow price feed 100 sec old, to avoid stale price feed errors


// pub const PROGRAM_AUTHOR: &str = "My Personal wallet key";
pub const MAX_COMPANY_WEIGHT: u8 = 10; // used by admin
// pub const MAX_REWARD: i64 = 100;
// pub const MAX_SLASH: i64 = 20;
// pub const MAX_USER_STATURE_UPDATES_BY_COMPANY: u64 = 100;
pub const ANCHOR_DISCRIMINATOR : usize = 8;
// pub const RATE_LIMIT_SECONDS: i64 = 300; // 5 minutes

// pub const HARD_CAP_USER_STATURE_RECORDS: u64 = 1000;
// pub const HARD_CAP_USER_STATURE_PER_COMPANY: u64 = 10000;
// pub const HARD_CAP_COMPANY_STATURE: u64 = 100000;