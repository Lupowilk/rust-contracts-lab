use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, Mint, MintTo, FreezeAccount, TokenAccount};
use anchor_spl::associated_token::AssociatedToken;
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

#[derive(Accounts)]
pub struct MintNft<'info> {

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        mint::decimals = 0,
        mint::authority = payer,
        mint::freeze_authority = payer,
    )]
    pub mint: Account<'info, Mint>,

    #[account(
        init,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = payer,
        associated_token::token_program = token_program,
    )]
    pub token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
}
