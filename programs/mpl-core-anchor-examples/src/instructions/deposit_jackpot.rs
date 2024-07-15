pub use crate::constants::*;
pub use crate::instructions::*;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::native_token::LAMPORTS_PER_SOL;
use anchor_lang::system_program;

#[derive(Accounts)]
pub struct DepositJackpot<'info> {
    #[account(mut, seeds = [JACKPOT_SEED.as_bytes()], bump)]
    pub jackpot_vault: Account<'info, JackpotVault>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> DepositJackpot<'info> {
    pub fn handler(ctx: Context<DepositJackpot>, amount: u64) -> Result<()> {
        let cpi_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.signer.to_account_info(),
                to: ctx.accounts.jackpot_vault.to_account_info(),
            },
        );
        system_program::transfer(cpi_context, amount * LAMPORTS_PER_SOL)?;

        msg!("Deposited {} SOL into the vault", amount);
        Ok(())
    }
}
