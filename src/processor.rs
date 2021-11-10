use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

use crate::instruction::EsrowInstruction;

pub struct Processor;
impl Processor {
    pub fn process(
        program_id: &Pubkey,
        account: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = EsrowInstruction::unpack(instruction_data)?;

        match instruction {
            EsrowInstruction::InitEsrow { amount } => {
                msg!("Instruction: InitEscrow");
                Self::process_init_esrow(account, amount, program_id)
            }
        }
    }

    fn process_init_esrow(
        account: &[AccountInfo],
        amount: u64,
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let initializer = next_account_info(account_info_iter)?;

        if !initializer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        Ok(())
    }
}
