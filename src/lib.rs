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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_dao_init() {
        let a = 12;
        let b = 12;
    }
}
