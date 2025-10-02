use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum CustomError {
    #[error("Max value reached")]
    MaxValue,
    #[error("Underflow")]
    Underflow,
    #[error("Invalid value")]
    InvalidValue,
}

impl From<CustomError> for ProgramError {
    fn from(e: CustomError) -> Self {
        ProgramError::Custom(e as u32)
    }
}