use std::convert::TryInto;
use solana_program::program_error::ProgramError;

use crate::error::EscrowError::InvalidInstruction;


pub enum EscrowInstruction {
    // Starts trade by creating and populating an escrow 
    // acct & transferring ownership of the given temp
    // token account to the PDA
    
    // Accts expected:
    // 0. [signer]
    // 1. [writable]
    // 2. [] Initiatlizer's token acct who will receive
    // 3. [writable] escrow acct
    // 4. [] rent sysvar
    // 5. [] token program
    InitEscrow {
        // amt party A expects to recieve of token Y
        amount: u64,
    }
}

impl EscrowInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        // takes tag as first byte and rest as remaining
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;

        // is the 1st byte 0, if so, we good, otherwise no
        Ok(match tag {
            0 => Self::InitEscrow {
                amount: Self::unpack_amount(rest)?
            },
            _ => return Err(InvalidInstruction.into()),
        })
    }

    fn unpack_amount(input: &[u8]) -> Result<u64, ProgramError> {
        let amount = input
            .get(..8)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(InvalidInstruction)?;
        Ok(amount)
    }
}
