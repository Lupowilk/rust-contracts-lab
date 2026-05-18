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
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.token_account.to_account_info(),
                authority: ctx.accounts.payer.to_account_info(),
            },
        );
        token::mint_to(cpi_ctx, 1)?;

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

    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub system_program: Program<'info,System>,

    /// CHECK: Metaplex will validate this account
    pub token_metadata_program: UncheckedAccount<'info>,

}
