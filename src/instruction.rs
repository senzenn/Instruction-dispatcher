use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize)]
pub enum CounterInstruction {
    Initialize,
    Increment(u64),
    Decrement(u64),
    Reset,
    Set(u64),
    NoOp,
}

impl CounterInstruction {
    pub fn unpack(input: &[u8]) -> Option<Self> {
        let (&tag, rest) = input.split_first()?;
        match tag {
            0 => Some(Self::Initialize),
            1 | 2 | 5 => {
                let v = u64::from_le_bytes(rest.get(..8)?.try_into().ok()?);
                match tag {
                    1 => Some(Self::Increment(v)),
                    2 => Some(Self::Decrement(v)),
                    _ => Some(Self::Set(v)),
                }
            }
            3 => Some(Self::Reset),
            6 => Some(Self::NoOp),
            _ => None,
        }
    }
}
