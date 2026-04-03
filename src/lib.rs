#![cfg_attr(not(test), no_std)]

use quasar_lang::prelude::*;
pub mod instructions;
pub use instructions::*;
declare_id!("GFg7f1owHQXYq9z6BvZYVpnJ2TAuh1GcDCGrrzuTAhJX");

#[program]
mod quasar_vault {
    use super::*;

    #[instruction(discriminator = 1)]
    pub fn desposit(ctx: Ctx<Deposit>, amount: u64) -> Result<(), ProgramError> {
        ctx.accounts.deposit(amount)
    }

    #[instruction(discriminator = 2)]
    pub fn withdraw(ctx: Ctx<Withdraw>, amount: u64) -> Result<(), ProgramError> {
        ctx.accounts.withdraw(amount)
    }
}

#[cfg(test)]
mod tests;
