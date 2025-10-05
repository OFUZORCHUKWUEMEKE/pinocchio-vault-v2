use std::vec;

use mollusk_svm::result::{Check, ProgramResult};
use mollusk_svm::{program, Mollusk};
use solana_sdk::account::Account;
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;

use pinocchio_escrow::instruction::{DepositInstructionData, WithdrawInstructionData};
use pinocchio_escrow::ID;

pub const PROGRAM: Pubkey = Pubkey::new_from_array(ID);

pub const PAYER: Pubkey = pubkey!("9vCdf2rh7hA7JdSVV1LEbJGFDNLjk1KHGTZW1wSRN6vC");

pub fn mollusk() -> Mollusk {
    let mollusk = Mollusk::new(&PROGRAM, "target/deploy/pinocchio_escrow.so");
    mollusk
}

const BASE_LAMPORTS: u64 = 10 * LAMPORTS_PER_SOL;
const DEPOSIT_AMOUNT: u64 = 1;
const DEPOSIT_LAMPORTS: u64 = DEPOSIT_AMOUNT * LAMPORTS_PER_SOL;

#[test]
fn test_deposit() {
    let mollusk = mollusk();

    let (system_program, system_account) = program::keyed_account_for_system_program();

    let (vault_pda, bump) =
        Pubkey::find_program_address(&[b"p-vault", &PAYER.to_bytes()], &PROGRAM);

    // prepare accounts
    let payer_account = Account::new(BASE_LAMPORTS, 0, &system_program);
    let vault_account = Account::new(0, 0, &system_program);

    let instruction_accounts = vec![
        AccountMeta::new(PAYER, true),
        AccountMeta::new(vault_pda, false),
        AccountMeta::new_readonly(system_program, false),
    ];

    let instruction_data = DepositInstructionData {
        bump,
        amount: DEPOSIT_AMOUNT.to_le_bytes(),
    };

    let mut ser_instruction_data = vec![0];

    ser_instruction_data.extend_from_slice(bytemuck::bytes_of(&instruction_data));

    let instruction =
        Instruction::new_with_bytes(PROGRAM, &ser_instruction_data, instruction_accounts);

    let transaction_accounts = &vec![
        (PAYER, payer_account.clone()),
        (vault_pda, vault_account.clone()),
        (system_program, system_account.clone()),
    ];

    let deposit_res = mollusk.process_and_validate_instruction(
        &instruction,
        transaction_accounts,
        &[
            Check::success(),
            Check::account(&PAYER)
                .lamports(BASE_LAMPORTS - DEPOSIT_LAMPORTS)
                .build(),
            Check::account(&vault_pda)
                .lamports(DEPOSIT_LAMPORTS)
                .build(),
        ],
    );

    assert!(deposit_res.program_result == ProgramResult::Success);
}
