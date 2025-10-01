use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone, PartialEq)]
pub enum MyError {
    #[error("Value cannot exceed 100.")]
    MaxValueReached,
}
