use crate::constants::*;
use crate::error::*;
use crate::DepositJackpot;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar::instructions;
use mpl_core::types::{DataState, PluginAuthorityPair};

#[derive(Accounts)]
pub struct CreateV1<'info> {
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
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct CreateV1Args {
    pub name: String,
    pub uri: String,
    // TODO: Add plugin_authority_pair if needed
    pub plugins: Option<Vec<PluginAuthorityPair>>,
}

impl<'info> CreateV1<'info> {
    pub fn handler(ctx: Context<CreateV1>, args: CreateV1Args) -> Result<()> {
        // check that payer account has more than JACKPOT_FEES lamport to pay for the jackpot fees
        let payer_balance = ctx.accounts.signer.lamports();
        if payer_balance < JACKPOT_FEES {
            return Err(WrapperError::InsufficientFunds.into());
        }

        /*
        TODO : marche pas !!!!!
        //create a DepositJAckpot context
        let ctx_deposit_jackpot = DepositJackpot {
            jackpot_vault: ctx.accounts.jackpot_vault.to_account_info(),
            signer: ctx.accounts.signer,
            system_program: ctx.accounts.system_program,
        };

        //call the deposit_jackpot(ctx: Context<DepositJackpot>, amount: u64) -> Result<()> function to deposit JACKPOT_FEES into the jackpot vault
        DepositJackpot::<'_>::handler(ctx_deposit_jackpot, JACKPOT_FEES)?;
        */

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

        Ok(())
    }
}
