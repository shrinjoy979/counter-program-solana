use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo}, entrypoint, entrypoint::{ProgramResult}, msg, pubkey::Pubkey
};

/* 
    next_account_info: Function for fetching the next account from the input accounts
    AccountInfo: Struct which represents what an input account looks like
    entrypoint!: macro that marks the program's entry function.
    msg!: logs to Solana runtime.
    ProgramResult: the result type returned by Solana programs.
    PubKey: Pubkey struct Represents a public key
*/

#[derive(BorshDeserialize, BorshSerialize)]

enum InstructionType {
    Increment(u32),
    Decrement(u32)
}

#[derive(BorshDeserialize, BorshSerialize)]
struct Counter {
    count: u32,
}

entrypoint!(counter_contract);

pub fn counter_contract(
    program_id: &Pubkey, // Public key of the deployed program (not used here)
    accounts: &[AccountInfo], // An array of input addresses. This array should have all the addresses the user is going to interact with when they are calling this function. 
    instruction_data: &[u8] // The actual `thing` the user wants to do. Its an array of bytes, but it can be deserialized into a struct. This contains information like what to do (add to a counter, remove from a counter, the value to add/remove)
    // u8 is unsigned 8 bit integer, means it has 8 places to store 0's and 1's, net decimal values it can store between 0 to 255
) -> ProgramResult {

    /*
        What does ? do?
        In plain English:
        "Try this. If it works, keep going.
        If it fails (returns an error), stop and return the error right away."
    */

    let acc = next_account_info(&mut accounts.iter())?; // Gets the first account in the list. This is expected to be the account holding the Counter

    let instruction_type = InstructionType::try_from_slice(instruction_data)?; // Deserialize the incoming instruction bytes into an InstructionType.

    let mut counter_data = Counter::try_from_slice(&acc.data.borrow())?; // Reads and deserializes the Counter struct from the account data.
    // you have to borrow, because no other thread can read from it.

    match instruction_type {
        InstructionType::Increment(value) => {
            msg!("Executing incress");
            counter_data.count += value;
        },

        InstructionType::Decrement(value) => {
            msg!("Executing decress");
            counter_data.count -= value;
        }
    }

    /*
        acc.data.borrow_mut() gives you a RefMut<[u8]>
        *acc.data.borrow_mut() dereferences the RefMut to get [u8]
    */

    counter_data.serialize(&mut *acc.data.borrow_mut()); // Writes the updated Counter back into the account data, Serialize the data back into the account.

    msg!("Contract succeded");

    Ok(())
}

