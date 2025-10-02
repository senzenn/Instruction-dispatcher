use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::{Counter, CounterInstruction, CustomError};

pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let instruction = CounterInstruction::unpack(data).ok_or(ProgramError::InvalidInstructionData)?;
    let account_iter = &mut accounts.iter();
    let account = next_account_info(account_iter)?;

    if account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    let mut counter = if account.data_len() > 0 {
        Counter::try_from_slice(&account.data.borrow())?
    } else {
        Counter { value: 0 }
    };

    match instruction {
        CounterInstruction::Initialize => counter.value = 0,
        CounterInstruction::Increment(v) => {
            counter.value = counter.value.checked_add(v).ok_or(CustomError::MaxValue)?
        }
        CounterInstruction::Decrement(v) => {
            counter.value = counter.value.checked_sub(v).ok_or(CustomError::Underflow)?
        }
        CounterInstruction::Reset => counter.value = 0,
        CounterInstruction::Set(v) => {
            if v > 100 {
                return Err(CustomError::InvalidValue.into());
            }
            counter.value = v
        }
        CounterInstruction::NoOp => {}
    }

    counter.serialize(&mut *account.data.borrow_mut())?;
    msg!("Counter: {}", counter.value);
    Ok(())
}