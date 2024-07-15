pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;
pub use constants::*;
pub use instructions::*;

declare_id!("BGDWR7vHYwqw43d43XjepHPYBfF2LVNqgLT6DFcYfkM9");

#[program]
pub mod mpl_core_anchor_wrapper {
    use super::*;

    pub fn create_nft_v1(ctx: Context<CreateNftV1>, args: CreateNftV1Args) -> Result<()> {
        create_nft_v1::CreateNftV1::handler(ctx, args)
    }

    pub fn create_collection_v1(
        ctx: Context<CreateCollectionV1>,
        args: CreateCollectionV1Args,
    ) -> Result<()> {
        create_collection_v1::CreateCollectionV1::handler(ctx, args)
    }

    pub fn transfer_v1(ctx: Context<TransferV1>, args: TransferV1Args) -> Result<()> {
        transfer_v1::TransferV1::handler(ctx, args)
    }

    pub fn deposit_jackpot(ctx: Context<DepositJackpot>, amount: u64) -> Result<()> {
        deposit_jackpot::DepositJackpot::handler(ctx, amount)
    }

    pub fn withdraw_jackpot(ctx: Context<WithdrawJackpot>, amount: u64) -> Result<()> {
        withdraw_jackpot::WithdrawJackpot::handler(ctx, amount)
    }

    // Fonction pour initialiser le vault
    pub fn init_jackpot_vault(ctx: Context<InitializeJackpotVault>) -> Result<()> {
        initialize_jackpot_vault::InitializeJackpotVault::handler(ctx)
    }
}
