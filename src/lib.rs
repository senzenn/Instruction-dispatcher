//  lib.rs // mainland modules connector

pub mod instruction; // 
pub mod entrypoint; // here goes to solana 
pub mod state;
pub mod error;
pub mod processor;
pub mod utils; // helpers functions 

// reexport
pub use instruction::*;

pub use state::*;
pub use error::*;

/// test for myself  
// create an account strcuts
use borsh::{BorshSerialize, BorshDeserialize};
#[derive(BorshDeserialize, BorshSerialize, Debug)]

pub struct SerialAccount {
    pub value: u64,
}
