use anchor_lang::prelude::*;
// use anchor_lang::system_program::{transfer, Transfer};
mod error;
use crate::error::ErrorCode;


mod constants;
use crate::constants::{ ANCHOR_DISCRIMINATOR, MAX_COMPANY_WEIGHT, MAX_REWARD, MAX_SLASH, MAX_USER_STATURE_UPDATES_BY_COMPANY, HARD_CAP_USER_STATURE_RECORDS, RATE_LIMIT_SECONDS, };

#[cfg(test)]
mod tests;



// 1. 🛑 Anti-abuse layer

// Right now:
// 👉 A company can still give +1M honor

// You’ll eventually need:

// Rate limits
// Stake/slashing
// Or weighting system

// 3. 🏆 Company reputation

// Eventually:

// Not all companies should have equal weight

// Think:

// Uber driver vs random user rating



declare_id!("9VFHpUQnHsG94AKzGfzf4mAeunxcQw8G9am6FfVEBVZb");


#[program]
pub mod stature {
    use super::*;

    pub fn initialize_config(ctx: Context<InitializeConfig>) -> Result<()> {
        let config = &mut ctx.accounts.config;

        require!(!config.is_initilized, ErrorCode::AlreadyInitialized);

        config.admin = ctx.accounts.admin.key();
        config.bump = ctx.bumps.config;
        config.is_initilized = true;

        msg!("✅ success: initialized admin keys for stature.network. Now {} can verify companies and update company status",config.admin,  );
        Ok(())
    }


    pub fn update_admin(ctx: Context<UpdateAdmin>, new_admin: Pubkey) -> Result<()> {
        let config = &mut ctx.accounts.config;
        let old_admin_key = config.admin;
        config.admin = new_admin;

        msg!("✅ success: changed admin from {} to {}", old_admin_key,  config.admin,  );
        Ok(())
    }


// -------------------------------
    // Update User Suspension (Admin Only)
    // -------------------------------
    pub fn update_user_suspension(ctx: Context<UpdateUserSuspension>, is_suspended: bool) -> Result<()> {
        require!(ctx.accounts.config.admin == ctx.accounts.admin.key(), ErrorCode::AdminActionOnly);


        let user = &mut ctx.accounts.user;
        user.is_suspended = is_suspended;

        msg!("🟢 user {} is_suspended: {}", user.key(), user.is_suspended );
        Ok(())
    }


    pub fn update_company_suspension(ctx: Context<UpdateCompany>, is_suspended: bool) -> Result<()> {
        require!(ctx.accounts.config.admin == ctx.accounts.admin.key(), ErrorCode::AdminActionOnly);

        let company = &mut ctx.accounts.company;
        company.is_suspended = is_suspended;

        msg!("🟢 company {} is_suspended: {}", company.name, company.is_suspended );
        Ok(())
    }

    pub fn update_company_weight(ctx: Context<UpdateCompany>, new_weight: u8 ) -> Result<()> { 
        require!(ctx.accounts.config.admin == ctx.accounts.admin.key(), ErrorCode::AdminActionOnly);
        require!(new_weight > 0 && new_weight <= MAX_COMPANY_WEIGHT, ErrorCode::InvalidWeight);

        let company = &mut ctx.accounts.company;

        // new-weight 10 = 1.0x, 50 = 5.0x
        // As the "World Record" judge, you set the weight based on 
        // their contribution to the "Wealth of Nations"
        // todo: company.weight = f(total_positive_raw_stature_given, trust_score, age)
        company.weight = new_weight; 
        
        msg!("🟢 updated company: {} weight: {}", company.name, company.weight);
        Ok(())
    }

    pub fn update_company_record_cap(
        ctx: Context<UpdateCompany>, 
        new_record_cap: u64 // 10 = 1.0x, 50 = 5.0x
    ) -> Result<()> {
        require!(ctx.accounts.config.admin == ctx.accounts.admin.key(), ErrorCode::AdminActionOnly);

        let company = &mut ctx.accounts.company;
        let old_cap  = company.max_record_cap;

        company.max_record_cap = new_record_cap; // todo: company.weight = f(total_positive_raw_stature_given, trust_score, age)
        
        msg!("🟢 company {} max record cap updated from {} to {}", company.name, old_cap, new_record_cap);
        Ok(())
    }


    pub fn update_company_verified_status(ctx: Context<UpdateCompany>) -> Result<()> {
        require!(ctx.accounts.config.admin == ctx.accounts.admin.key(), ErrorCode::AdminActionOnly);

        let company = &mut ctx.accounts.company;
        company.is_verified = true;
        company.admin = ctx.accounts.admin.key();// The admin who verifies it becomes the recorded admin for this company

        msg!("✅ success: company {} is now verified. company rep {} can now update user statures. company can now be monitored, suspended, and updated by admin {}", company.name,  company.company_rep, company.admin);
        Ok(())
    }

// used to punish company for bad behaviour
    pub fn update_company_stature(ctx: Context<UpdateCompany>, new_stature: i64) -> Result<()> {
        require!(ctx.accounts.config.admin == ctx.accounts.admin.key(), ErrorCode::AdminActionOnly);

        let company = &mut ctx.accounts.company;
        company.stature = new_stature;

        msg!("🟢 updated company: {} stature: {}", company.name, company.stature);
        Ok(())
    }



    // -------------------------------
    // Initialize Company
    // -------------------------------
    pub fn initialize_company(ctx: Context<InitializeCompany>,name: String) -> Result<()> {
        require!(name.len() <= 32, ErrorCode::StringTooLong);

        let company = &mut ctx.accounts.company;
        company.admin = Pubkey::default(); // 👈 no admin yet
        company.company_rep = ctx.accounts.company_rep.key();
        company.name = name;
        company.record_count = 0;
        company.total_negative_raw = 0;
        company.total_positive_raw = 0;
        company.weight = 1;
        company.max_record_cap = MAX_USER_STATURE_UPDATES_BY_COMPANY;
        company.is_verified  = false; 
        company.is_suspended = false;
        company.bump = ctx.bumps.company;


        msg!("✅ success: initialized company {} by {}. Waiting for varification by admin.", company.name, company.company_rep);
        Ok(())
    }

    // -------------------------------
    // Initialize User
    // -------------------------------
    pub fn initialize_user(ctx: Context<InitializeUser>, name: String) -> Result<()> {
        require!(name.len() <= 32, ErrorCode::StringTooLong);

        let user = &mut ctx.accounts.user;
        user.id = ctx.accounts.owner.key();
        user.name = name;
        user.stature = 0;
        user.record_idx = 0;
        user.is_suspended = false;
        user.bump = ctx.bumps.user;

        msg!("✅ success: initialized user {} by {}", user.name, user.id);
        Ok(())
    }

    // -------------------------------
    // Update Honor (CORE LOGIC)
    // -------------------------------
    pub fn update_user_stature(
        ctx: Context<UpdateUserStature>,
        amount: i64, 
        // db_id: u64, 
        nonce: u64, // 👈 or [u8; 16] for UUID
    ) -> Result<()> {
        let user = &mut ctx.accounts.user;
        let record = &mut ctx.accounts.record;
        let company = &mut ctx.accounts.company;
        let company_user_state = &mut ctx.accounts.company_user_state;

        let now = Clock::get()?.unix_timestamp;

        require!(!user.is_suspended, ErrorCode::UserSuspended);
        require!(user.id != company.company_rep, ErrorCode::CannotSelfAssignStature);

        require!(!company.is_suspended, ErrorCode::CompanySuspended);
        require!(company.is_verified, ErrorCode::CompanyNotVerified);
        require!(company.record_count < company.max_record_cap, ErrorCode::RequestLimitIncrease);
        // require!(company_user_state.db_id != db_id, ErrorCode::InvalidRecord);
        require!(nonce > company_user_state.last_nonce, ErrorCode::InvalidNonce);
        require!(company.weight > 0, ErrorCode::InvalidWeight);

        require!(amount >= -MAX_SLASH && amount <= MAX_REWARD, ErrorCode::TooMuch);
        require!(user.record_idx < HARD_CAP_USER_STATURE_RECORDS, ErrorCode::TooManyUpdates);

        // 2. Rate Limiting
        if company_user_state.last_updated_at != 0 { // accounts for initialization when last_updated_at = 0
            require!(
                now - company_user_state.last_updated_at > RATE_LIMIT_SECONDS,
                ErrorCode::RateLimited
            );
        }


        // require!(nonce > company_user_state.last_nonce, ErrorCode::InvalidNonce);
        // company_user_state.last_nonce = nonce;

        // -------------------------------
        // SAFETY: Prevent overflow
        // -------------------------------

        // Logic: User Effort (amount) * Company Utility (weight)
        // 3. Stature Scaling Logic
        let weight_multiplier = company.weight as i64;
        let weighted_honor = amount.checked_mul(weight_multiplier).ok_or(ErrorCode::Overflow)?;

        // Anti-inflation: Larger existing stature makes gaining/losing more stature harder
        let value = user.stature.abs() as u64 + 1;
        let scale = (64 - value.leading_zeros()) as i64;

        let adjusted_honor = weighted_honor
            .checked_div(scale.max(1))
            .ok_or(ErrorCode::Overflow)?;
        msg!("🟢 calculated new honor. amount: {}, company_weight: {}, weighted_honor: {}, adjusted_honor:{}", amount, company.weight, weighted_honor, adjusted_honor);


        // 4. Update User Account
        user.stature = user
            .stature
            .checked_add(adjusted_honor)
            .ok_or(ErrorCode::Overflow)?;
        msg!("🟢 updated user stature to {}", user.stature);



        // 5. Update Company Stats
        msg!("before updating... company_record_count: {}, positives: {}, negatives: {}", company.record_count, company.total_positive_raw, company.total_negative_raw );

        if amount > 0 {
            company.total_positive_raw = company
                .total_positive_raw
                .checked_add(amount)
                .ok_or(ErrorCode::Overflow)?;


            // Only count positive contributions toward company reputation
            // Company stature = total positive value distributed
            // Companies are measured based on value generated
            company.stature = company
                .stature
                .checked_add(adjusted_honor)
                .ok_or(ErrorCode::Overflow)?;
         } else {
            company.total_negative_raw = company
                .total_negative_raw
                .checked_add(amount.abs())
                .ok_or(ErrorCode::Overflow)?;
        }

        company.record_count = company
            .record_count
            .checked_add(1)
            .ok_or(ErrorCode::Overflow)?;
       msg!("after updating... company_record_count: {}, positives: {}, negatives: {}", company.record_count, company.total_positive_raw, company.total_negative_raw );

        // -------------------------------
        // Create immutable record
        // ------------------------------- 
     
     // 6. Update Mappings and State
        company_user_state.last_updated_at = now;
        //company_user_state.db_id = db_id;
        company_user_state.last_nonce = nonce;
        company_user_state.bump = ctx.bumps.company_user_state;
        msg!("🟢 created new Stature Record");
        

        let idx = user.record_idx;
            // increment AFTER using it
        user.record_idx = user
            .record_idx
            .checked_add(1)
            .ok_or(ErrorCode::Overflow)?;

        record.company = company.key();
        record.user = user.key();
        record.amount = adjusted_honor;
        record.timestamp = now;
        record.user_record_idx = idx;
        record.bump = ctx.bumps.record;
    
  
       msg!("✅ success: updated user stature. user: {} id: {} stature: {}", user.name, user.id, user.stature);
       emit!(UpdateUserEvent {
            user: user.id,
            company: company.key(),
            amount: adjusted_honor.try_into().map_err(|_| ErrorCode::Overflow)?,
            timestamp: record.timestamp, 
        });
        Ok(())
    }

    // -------------------------------
    // Get Honor
    // -------------------------------
    pub fn get_user_stature(ctx: Context<GetUserStature>) -> Result<i64> {
        let user = &ctx.accounts.user;
        msg!("🟢 Current stature for {}: {}", user.name, user.stature);
        Ok(user.stature)
    }
}



#[derive(Accounts)]
pub struct UpdateUserSuspension<'info> {
    pub admin: Signer<'info>,

    #[account(
        seeds = [b"config"],
        bump = config.bump,
        has_one = admin
    )]
    pub config: Account<'info, Config>,

    #[account(
        mut, 
        seeds = [b"user", user.id.as_ref()], 
        bump = user.bump
    )]
    pub user: Account<'info, User>,
}

#[derive(Accounts)]
pub struct UpdateCompany<'info> {
    pub admin: Signer<'info>,
    #[account(
        seeds = [b"config"],
        bump = config.bump,
        has_one = admin
    )]
    pub config: Account<'info, Config>,

    #[account(
        mut, 
        seeds = [b"company", company.company_rep.as_ref()], 
        bump = company.bump, 
    )]
    pub company: Account<'info, Company>,
}





// Separate context for reading
#[derive(Accounts)]
pub struct GetUserStature<'info> {
    #[account(
        seeds = [b"user", user.id.as_ref()],
        bump = user.bump
    )]
    pub user: Account<'info, User>,
}

#[derive(Accounts)]
// #[instruction(nonce: u64)]
pub struct UpdateUserStature<'info> {
    #[account(mut)]
    pub company_rep: Signer<'info>, 

    #[account(
        mut,
        seeds = [b"company", company.company_rep.as_ref()], 
        bump = company.bump,
        has_one = company_rep
    )]
    pub company: Account<'info, Company>,

    #[account(
        mut,
        seeds = [b"user", user.id.as_ref()],
        bump = user.bump
    )]
    pub user: Account<'info, User>,


    #[account(
        init_if_needed,
        payer = company_rep,
        space = ANCHOR_DISCRIMINATOR + CompanyUserState::INIT_SPACE, // 8 + 1, // last_updated_at + bump
        seeds = [
            b"company_user_state",
            company.key().as_ref(),
            user.key().as_ref(), 
        ],
        bump
    )]
    pub company_user_state: Account<'info, CompanyUserState>,


    #[account(
        init,
        payer = company_rep,
        space = ANCHOR_DISCRIMINATOR + StatureRecord::INIT_SPACE,
        seeds = [
            b"stature_record",
            user.key().as_ref(),
            &user.record_idx.to_le_bytes(), // 👈 KEY CHANGE
          //  &nonce.to_le_bytes(), // gives randomness but no indexing capabilities
        ],
        bump
    )]
    pub record: Account<'info, StatureRecord>,

    pub system_program: Program<'info, System>,
}


// for emiting event
#[event]
pub struct UpdateUserEvent {
    pub user: Pubkey,
    pub company: Pubkey,
    pub amount: i64, 
    pub timestamp: i64
}




#[derive(Accounts)]
pub struct InitializeCompany<'info> {
    #[account(mut)]
    pub company_rep:Signer<'info>, 

    #[account(
        init,
        payer = company_rep,
        space = ANCHOR_DISCRIMINATOR + Company::INIT_SPACE,
        seeds = [b"company", company_rep.key().as_ref()],
        bump
    )]
    pub company: Account<'info, Company>,

    pub system_program: Program<'info, System>,
}




#[derive(Accounts)]
pub struct InitializeUser<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = ANCHOR_DISCRIMINATOR + User::INIT_SPACE,
        seeds = [b"user", owner.key().as_ref()],
        bump
    )]
    pub user: Account<'info, User>,

    pub system_program: Program<'info, System>,
}





#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(mut)]
    pub admin: Signer<'info>, // right now its just my wallet - // todo: upgrade to multi sig

    #[account(
        init,
        payer = admin,
        space = ANCHOR_DISCRIMINATOR + Config::INIT_SPACE,
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, Config>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateAdmin<'info> {
  #[account(mut)]
    pub admin: Signer<'info>, // current admin

    #[account(
        mut,
        seeds = [b"config"],
        bump = config.bump,
        has_one = admin // 🔒 only current admin can update
    )]
    pub config: Account<'info, Config>,
}


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

