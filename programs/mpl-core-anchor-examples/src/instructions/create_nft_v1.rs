use crate::constants::*;
use crate::error::*;
use crate::JackpotVault;
use anchor_lang::prelude::*;
use anchor_lang::system_program;
use mpl_core::types::{DataState, PluginAuthorityPair};
use num_traits::ToBytes;
use rand_chacha::rand_core::RngCore;
use rand_chacha::ChaChaRng; // Add this line to import the deposit function
use rand_chacha::rand_core::SeedableRng;


#[derive(Accounts)]
pub struct CreateNftV1<'info> {
    /// The address of the new asset.
    #[account(mut)]
    pub asset: Signer<'info>,

    /// The collection to which the asset belongs.
    /// CHECK: Checked in mpl-core.
    #[account(mut)]
    pub collection: Option<AccountInfo<'info>>,

    /// The authority signing for creation.
    pub authority: Option<Signer<'info>>,

    /// The account paying for the storage fees.
    #[account(mut)]
    pub signer: Signer<'info>,

    /// The owner of the new asset. Defaults to the authority if not present.
    /// CHECK: Checked in mpl-core.
    pub owner: Option<AccountInfo<'info>>,

    /// The authority on the new asset.
    /// CHECK: Checked in mpl-core.
    pub update_authority: Option<AccountInfo<'info>>,

    /// The system program.
    pub system_program: Program<'info, System>,

    /// The SPL Noop program.
    /// CHECK: Checked in mpl-core.
    pub log_wrapper: Option<AccountInfo<'info>>,

    /// The MPL Core program.
    /// CHECK: Checked in mpl-core.
    #[account(address = mpl_core::ID)]
    pub mpl_core: AccountInfo<'info>,

    //JACKPOT vault account
    #[account(
        mut, 
        seeds = [JACKPOT_SEED.as_bytes()], 
        bump)]
    pub jackpot_vault: Account<'info, JackpotVault>,
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct CreateNftV1Args {
    pub name: String,
    pub uri: String,
    // TODO: Add plugin_authority_pair if needed
    pub plugins: Option<Vec<PluginAuthorityPair>>,
}

impl<'info> CreateNftV1<'info> {
    pub fn handler(ctx: Context<CreateNftV1>, args: CreateNftV1Args) -> Result<()> {
        // check that payer account has more than JACKPOT_FEES lamport to pay for the jackpot fees
        let payer_balance = ctx.accounts.signer.lamports();
        if payer_balance < JACKPOT_FEES {
            return Err(WrapperError::InsufficientFunds.into());
        }

        let cpi_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.signer.to_account_info(),
                to: ctx.accounts.jackpot_vault.to_account_info(),
            },
        );

        system_program::transfer(cpi_context, JACKPOT_FEES)?;

        //msg!("Deposited {} lamports into the vault", JACKPOT_FEES);

        mpl_core::instructions::CreateV1Cpi {
            asset: &ctx.accounts.asset.to_account_info(),
            collection: ctx.accounts.collection.as_ref(),
            authority: ctx.accounts.authority.as_deref(),
            payer: &ctx.accounts.signer.to_account_info(),
            owner: ctx.accounts.owner.as_ref(),
            update_authority: ctx.accounts.update_authority.as_ref(),
            system_program: &ctx.accounts.system_program.to_account_info(),
            log_wrapper: ctx.accounts.log_wrapper.as_ref(),
            __program: &ctx.accounts.mpl_core,
            __args: mpl_core::instructions::CreateV1InstructionArgs {
                data_state: DataState::AccountState,
                name: args.name,
                uri: args.uri,
                plugins: args.plugins,
            },
        }
        .invoke()?;

        let clock: std::result::Result<Clock, ProgramError> = Clock::get();
        let current_timestamp = match clock {
            Ok(clock) => clock.unix_timestamp,
            Err(_) => {
                // Handle the error case 
                return Err(WrapperError::ClockRetrievalFailed.into());
            }
        };
        msg!("current_timestamp {:?}", current_timestamp);

        let timestamp_bytes = current_timestamp.to_le_bytes(); 
        let array_32: [u8; 32] = timestamp_bytes.repeat(4).try_into().unwrap();
        let mut rng = ChaChaRng::from_seed(array_32);

        //get a randmom number
        let rand_integer: u32 = rng.next_u32();
        msg!("rand_integer {:?}", rand_integer);

        //get a number from 0 to JACKPOT_MAX_TICKETS-1 
        let random = rand_integer % JACKPOT_MAX_TICKETS ;
        msg!("random {:?}", random);
           

        /*  
        //convert payer account pubkey to u32
        let payer = ctx
            .accounts
            .signer
            .key
            .to_bytes()
            .iter()
            .fold(0, |acc, &x| acc * 256 + x as u32);
        msg!("payer {}", payer);
        */

        /* 
        // seed = epoch xor payer
        let seed = epoch ^ payer;
        // trunc the seed betwwen 0 and 6
        let seed: u32 = seed % (JACKPOT_MAX_TICKETS + 1);
        */

        //let seed = (epoch ^ payer)% (JACKPOT_MAX_TICKETS + 1);
        /* 
        let seed = ((Clock::get().unwrap().epoch as u32)
            ^ (ctx
                .accounts
                .signer
                .key
                .to_bytes()
                .iter()
                .fold(0, |acc, &x| acc * 256 + x as u32)))
            % (JACKPOT_MAX_TICKETS + 1);
        */
        //msg!("Seed {:?}", seed);
        
        Ok(())
    }
}
