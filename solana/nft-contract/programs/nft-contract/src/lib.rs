use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, Mint, MintTo, FreezeAccount};
use mpl_token_metadata::instructions::{
    CreateMetadataAccountV3Cpi,
    CreateMetadataAccountV3CpiAccounts,
    CreateMetadataAccountV3InstructionArgs,
};
use mpl_token_metadata::types::DataV2;
use mpl_token_metadata::ID as METADATA_PROGRAM_ID;

declare_id!("CtVesYJKLZQMhVShNtLkNcqVgMQotbmmb1qvaaGf3AEA");

#[program]
pub mod nft_contract {
    use super::*;

    pub fn mint_nft(
        ctx: Context<MintNft>,
        name: String,
        symbol: String,
        uri: String,
    ) -> Result<()> {
        Ok(())
    }
}
