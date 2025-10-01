use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone, PartialEq)]
pub enum CustomError {
    #[error("Value cannot exceed 100.")]
    MaxValueReached,

    #[error("cannot decremnt value to zero ")]
    CannotDecrementValue,

    #[error("Set vlaue cannot exceed 100 ")]
    SetValueTooHigh,
}

// converter

//impl CustomError for ProgramError{
//
//}
