    use std::mem::size_of;

pub use crate::constants::*;
pub use crate::instructions::*;
use anchor_lang::prelude::*;

    // Structure du compte Vault
    #[account]
    pub struct JackpotVault {    
        pub deposited_fee_number: u32,
    }

    // Définition du contexte pour l'initialisation du vault
    #[derive(Accounts)]
    pub struct InitializeJackpotVault<'info> {
        #[account(
            init, 
            payer = signer, 
            space = size_of::<JackpotVault>() + 8 +100, 
            seeds = [JACKPOT_SEED.as_bytes()], 
            bump)]
        pub jackpot_vault: Account<'info, JackpotVault>,
        #[account(mut)]
        pub signer: Signer<'info>,
        pub system_program: Program<'info, System>,
    }

    impl<'info> InitializeJackpotVault<'info> {
        pub fn handler(_ctx: Context<InitializeJackpotVault>) -> Result<()> {
            // initialize draw number to 0
            _ctx.accounts.jackpot_vault.deposited_fee_number = 0;
            msg!("Jackpot Vault created");
            msg!("Jackpot Vault address: {:?}", _ctx.accounts.jackpot_vault.key());
            Ok(())
        }
    }
 
