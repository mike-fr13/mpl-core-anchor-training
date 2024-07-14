    pub use crate::constants::*;
pub use crate::instructions::*;
use anchor_lang::prelude::*;

    // Structure du compte Vault
    #[account]
    pub struct JackpotVault {    }

    // Définition du contexte pour l'initialisation du vault
    #[derive(Accounts)]
    pub struct InitializeJackpotVault<'info> {
        #[account(
            init, 
            payer = signer, 
            space = 8 + 8, 
            seeds = [b"jackpot_vault"], 
            bump)]
        pub jackpot_vault: Account<'info, JackpotVault>,
        #[account(mut)]
        pub signer: Signer<'info>,
        pub system_program: Program<'info, System>,
    }

    impl<'info> InitializeJackpotVault<'info> {
        pub fn handler(_ctx: Context<InitializeJackpotVault>) -> Result<()> {
            msg!("Jackpot Vault created");
            msg!("Jackpot Vault address: {:?}", _ctx.accounts.jackpot_vault.key());
            Ok(())
        }
    }
 
