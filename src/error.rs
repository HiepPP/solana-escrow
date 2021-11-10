use solana_program::program_error::ProgramError;
use thiserror::Error;

pub enum EscrowError {
    #[error("Invalid Instruction")]
    InvalidInstruction,
}

impl From<EsrowError> for ProgramError {
    fn from(err: EsrowError) -> Self {
        ProgrammerError::Custom(e as u32)
    }
}
