use quasar_lang::prelude::*;

#[derive(Accounts)]
pub struct Withdraw<'info> {
    pub signer: &'info mut Signer,
    #[account(mut, seeds = [b"vault", signer], bump)]
    pub vault: &'info mut UncheckedAccount,
}

impl<'info> Withdraw<'info> {
    #[inline(always)]
    pub fn withdraw(&self, amount: u64) -> Result<(), ProgramError> {
        let vault = self.vault.to_account_view();
        let signer = self.signer.to_account_view();
        set_lamports(vault, vault.lamports() - amount);
        set_lamports(signer, signer.lamports() + amount);
        Ok(())
    }
}
