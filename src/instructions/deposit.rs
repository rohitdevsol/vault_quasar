use quasar_lang::prelude::*;

#[derive(Accounts)]
pub struct Deposit<'info> {
    pub signer: &'info mut Signer,
    #[account(mut, seeds = [b"vault", signer], bump)]
    pub vault: &'info mut UncheckedAccount,
    pub system_program: &'info Program<System>,
}

impl<'info> Deposit<'info> {
    #[inline(always)]
    pub fn deposit(&mut self, amount: u64) -> Result<(), ProgramError> {
        self.system_program.transfer(self.signer, self.vault, amount).invoke()?;
        Ok(())
    }
}
