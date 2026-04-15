use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Arithmetic overflow while updating values")]
    Overflow,

    #[msg("Company is suspended and cannot perform this action")]
    CompanySuspended,

    #[msg("Company must be verified before updating user stature")]
    CompanyNotVerified,
          
    #[msg("User is suspended and cannot receive updates")]
    UserSuspended,

    #[msg("Company weight out of bounds; must be between 0 and 10. verification or admin action required")]
    InvalidWeight, 

    #[msg("Invalid nonce: must be strictly increasing to prevent replay attacks")]
    InvalidNonce, 

    #[msg("Rate limit exceeded: please wait before updating this user again")]
    RateLimited, 

    #[msg("Invalid or malformed stature record")]
    InvalidRecord, 

    #[msg("Unauthorized: only the program admin can perform this action")]
    AdminActionOnly, 

    #[msg("Company has reached its update limit; request a cap increase from admin")]
    RequestLimitIncrease, 

    #[msg("Stature update amount exceeds allowed bounds")]
    TooMuch,

    #[msg("Input string is too long. Use a shorter name.")]
    StringTooLong, 


    #[msg("Cannot assign stature to yourself via your company")]
    CannotSelfAssignStature, 

    #[msg("Cannot re-initialize account after already intialized once")]
    AlreadyInitialized, 

    #[msg("Reached company to user stature update hard cap")]
    TooManyUpdates
}