use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

#[derive(BorshDeserialize, BorshSerialize)]
pub enum InstructionType {
    Init,
    Half,
    Double,
    Add(u32),
    Subtract(u32),
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct OnChainData {
    pub count: u32,
}

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let acc = next_account_info(account_info_iter)?;

    let instruction_type = InstructionType::try_from_slice(instruction_data)?;
    let mut counter_data = OnChainData::try_from_slice(&acc.data.borrow())?;

    match instruction_type {
        InstructionType::Init => {
            msg!("Initializing counter to 1");
            counter_data.count = 1;
        }
        InstructionType::Double => {
            msg!("Doubling the value");
            counter_data.count *= 2;
        }
        InstructionType::Half => {
            msg!("Dividing the value by half");
            counter_data.count /= 2;
        }
        InstructionType::Add(value) => {
            msg!("Adding {}", value);
            counter_data.count += value;
        }
        InstructionType::Subtract(value) => {
            msg!("Subtracting {}", value);
            counter_data.count -= value;
        }
    }

    counter_data.serialize(&mut *acc.data.borrow_mut())?;
    msg!("Updated counter: {}", counter_data.count);

    Ok(())
}
