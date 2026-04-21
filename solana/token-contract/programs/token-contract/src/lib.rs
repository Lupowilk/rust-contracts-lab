use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, Mint, InitializeMint, MintTo};

declare_id!("7XF9upHxdV5iVWehmVaAsPzzDwyrozdtgpEFFHbBtDTj");


#[program]
pub mod token_contract {
    use super::*;

    pub fn create_mint(ctx:Context<CreateMint>, decimals: u8) -> Result<()> {
        Ok(())
    }

    pub fn mint_tokens(ctx: Context<MintTokens>, amount: u64) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateMint<'info> {

    #[account(
        init,
        payer = payer,
        mint::decimals = decimals,
        mint::authority = payer,
    )]
    pub mint: Account<'info, Mint>, // a reference to an on-chain account whose data is a Mint.

    #[account(mut)]
    pub payer: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info,System>,
    pub rent: Sysvar<'info, Rent>

}
