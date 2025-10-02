use std::u64;

use borsh::{BorshSerialize, BorshDeserialize};

#[derive(BorshDeserialize, BorshSerialize)]
pub enum CounterInstruction {
    Initialize,
    Increment(u64),
    Decrement(u64),
    Reset,
    Set(u64),
    NoOps, // no operation
}

impl CounterInstruction {
    pub fn unpack(input: &[u8]) -> Option<CounterInstruction> {
        let tag = input.get(0)?; // extract data like Some(value) if not exist then None 

        match tag {
            0 => Some(CounterInstruction::Initialize),
            1 => {
                if input.len() < 9 {
                    return None;
                }
                let value = u64::from_le_bytes(input[1..9].try_into().ok()?);
                Some(CounterInstruction::Increment(value))
            }
            2 => {
                if input.len() < 9 {
                    return None;
                }
                let value = u64::from_le_bytes(input[1..9].try_into().ok()?);
                Some(CounterInstruction::Decrement(value))
            }
            4 => {
                if input.len() < 9 {
                    let value = u64::from_le_bytes(input[1..9].try_into().ok()?);
                    Some(CounterInstruction::Reset)
                }
            }
            3 => Some(CounterInstruction::Reset),
            _ => None,
        }
    }
}
