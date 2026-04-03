extern crate std;
use quasar_svm::{ Instruction, Pubkey, QuasarSvm };
use solana_address::Address;

use quasar_vault_client::DespositInstruction;
use quasar_vault_client::WithdrawInstruction;

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
        &[quasar_svm::token::create_keyed_system_account(&user, 10_000_000_000)]
    );

    result.assert_success();

    let vault_after = result.account(&Pubkey::from(vault));

    assert!(vault_after.is_none())
}
