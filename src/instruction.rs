use crate::error::EscrowError::InvalidInstruction;
use solana_programm::program_error::ProgramError;
use std::convert::TryInTo;

pub enum EsrowInstruction {
    InitEsrow { amount: u64 },
}

impl EsrowInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        Ok(match tag {
            0 => Self::InitEsrow {
                amount: Self::unpack_amount(rest)?,
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
