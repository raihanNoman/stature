use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Arithmetic overflow while updating values")]
    Overflow,

    #[msg("Registered Program is suspended and cannot perform this action")]
    ProgramSuspended,

    #[msg("Registered Program must be verified before updating user stature")]
    ProgramNotVerified,

    #[msg("User is suspended and cannot receive updates")]
    UserSuspended,

    #[msg("Registered Program weight out of bounds; must be between 0 and 10. verification or admin action required")]
    InvalidWeight,

    #[msg("The source account must be owned by the registered program")]
    InvalidSourceOwner,

    #[msg("Rate limit exceeded: please wait before updating this user again")]
    RateLimited,

    #[msg("Invalid or malformed stature record")]
    InvalidRecord,

    #[msg("Unauthorized: only the program admin can perform this action")]
    AdminActionOnly,

    #[msg("Registered Program has reached its update limit; request a cap increase from admin")]
    RequestLimitIncrease,

    #[msg("Stature update amount exceeds allowed bounds")]
    TooMuch,

    #[msg("Input string is too long. Use a shorter name.")]
    StringTooLong,

    #[msg("Cannot assign stature to yourself via your Registered Program")]
    CannotSelfAssignStature,

    #[msg("Cannot re-initialize account after already intialized once")]
    AlreadyInitialized,

    #[msg("Reached company to user stature update hard cap")]
    TooManyUpdates,
}
