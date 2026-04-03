use quasar_lang::prelude::*;

#[derive(Accounts)]
pub struct Withdraw<'info> {
    pub signer: &'info mut Signer,
    #[account(mut, seeds = [b"vault", signer], bump)]
    pub vault: &'info mut UncheckedAccount,
    pub system_program: &'info Program<System>,
}

impl<'info> Withdraw<'info> {
    #[inline(always)]
    pub fn withdraw(&self, amount: u64, seeds: [Seed<'_>; 3]) -> Result<(), ProgramError> {
        self.system_program.transfer(self.vault, self.signer, amount).invoke_signed(&seeds)?;
        Ok(())

        // If it was a program owned account we could have done it

        // let vault = self.vault.to_account_view();
        // let signer = self.signer.to_account_view();
        // set_lamports(vault, vault.lamports() - amount);
        // set_lamports(signer, signer.lamports() + amount);
    }
}
