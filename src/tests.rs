extern crate std;
use quasar_svm::{ Instruction, Pubkey, QuasarSvm };
use solana_address::Address;

use quasar_vault_client::DespositInstruction;
use quasar_vault_client::WithdrawInstruction;

use crate::withdraw;

fn setup() -> QuasarSvm {
    let elf = include_bytes!("../target/deploy/quasar_vault.so");
    QuasarSvm::new().with_program(&Pubkey::from(crate::ID), elf)
}

#[test]
fn test_deposit() {
    let mut svm = setup();

    let user = Pubkey::new_unique();
    let (vault, _vault_bump) = Address::find_program_address(
        &[b"vault", user.as_ref()],
        &crate::ID
    );

    let instruction: Instruction = (DespositInstruction {
        signer: Address::from(user.to_bytes()),
        vault,
        system_program: Address::from(quasar_svm::system_program::ID.to_bytes()),
        amount: 1_000_000_000,
    }).into();

    let result = svm.process_instruction(
        &instruction,
        &[
            quasar_svm::token::create_keyed_system_account(&user, 10_000_000_000),
            quasar_svm::token::create_keyed_system_account(&vault, 0),
        ]
    );

    result.assert_success();

    let vault_after = result.account(&Pubkey::from(vault.to_bytes())).unwrap();
    assert_eq!(vault_after.lamports, 1_000_000_000);
}

#[test]
fn test_withdraw() {
    let mut svm = setup();

    let user = Pubkey::new_unique();
    let (vault, _vault_bump) = Address::find_program_address(
        &[b"vault", user.as_ref()],
        &crate::ID
    );

    let initial_accounts = vec![
        quasar_svm::token::create_keyed_system_account(&user, 10_000_000_000),
        quasar_svm::token::create_keyed_system_account(&vault, 0)
    ];
    let deposit_ix: Instruction = (DespositInstruction {
        signer: Address::from(user.to_bytes()),
        vault,
        system_program: Address::from(quasar_svm::system_program::ID.to_bytes()),
        amount: 1_000_000_000,
    }).into();

    let dep_results = svm.process_instruction(&deposit_ix, &initial_accounts);

    // let vault_after = dep_results.account(&Pubkey::from(vault)).unwrap();
    // let user_after = dep_results.account(&Pubkey::from(user)).unwrap();
    // eprintln!("Vault after deposit: {:#?}", vault_after);
    // eprintln!("User after deposit: {:#?}", user_after);

    dep_results.assert_success();

    let next_accounts = dep_results.accounts;

    // now let us withdraw from the vault
    // user and vault is same just use a different instruction

    let withdraw_ix: Instruction = (WithdrawInstruction {
        amount: 1_000_000_000,
        signer: user,
        vault,
        system_program: Address::from(quasar_svm::system_program::ID.to_bytes()),
    }).into();

    let withdraw_results = svm.process_instruction(&withdraw_ix, &next_accounts);
    withdraw_results.assert_success();

    let vault_after1 = withdraw_results.account(&Pubkey::from(vault)).unwrap();
    let user_after1 = withdraw_results.account(&Pubkey::from(user)).unwrap();
    // the balance in the vault should be 0
    eprintln!("Vault1 after deposit: {:#?}", vault_after1);
    eprintln!("User1 after deposit: {:#?}", user_after1);
    assert_eq!(vault_after1.lamports, 0);
    assert_eq!(user_after1.lamports, 10_000_000_000);
}
