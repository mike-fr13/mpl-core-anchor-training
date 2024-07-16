pub mod constants;
pub mod error;
pub mod instructions;

use anchor_lang::prelude::*;
pub use constants::*;
pub use instructions::*;

declare_id!("AJLzryg4vZYhxHUYpVLrej8LcXd51tsgsosfgGR8WNT3");

#[program]
pub mod nft_lotery_no_code {
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

    // Fonction pour initialiser le vault
    pub fn init_jackpot_vault(ctx: Context<InitializeJackpotVault>) -> Result<()> {
        initialize_jackpot_vault::InitializeJackpotVault::handler(ctx)
    }
}
