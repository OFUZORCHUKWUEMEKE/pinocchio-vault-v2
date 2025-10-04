use crate::instruction::{self, VaultInstruction};
use pinocchio::{
    account_info::AccountInfo, no_allocator, nostd_panic_handler, program_entrypoint,
    program_error::ProgramError, pubkey::Pubkey, ProgramResult,
};

program_entrypoint!(process_instruction);
no_allocator!();

nostd_panic_handler!();

#[inline(always)]
fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let (ix_disc, instruction_data) = instruction_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match VaultInstruction::try_from(ix_disc)? {
        VaultInstruction::Deposit => instruction::process_deposit(accounts, instruction_data),
        VaultInstruction::Withdraw => instruction::process_withdraw(accounts, instruction_data),
    }
}
