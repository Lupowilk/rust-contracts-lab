use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, Mint, MintTo, TokenAccount};

declare_id!("7XF9upHxdV5iVWehmVaAsPzzDwyrozdtgpEFFHbBtDTj");


#[program]
pub mod token_contract {
    use super::*;

    pub fn create_mint(ctx:Context<CreateMint>, decimals: u8) -> Result<()> {
         Ok(())
    }

    pub fn mint_tokens(ctx: Context<MintTokens>, amount: u64) -> Result<()> {
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.receiver_account.to_account_info(),
                    authority: ctx.accounts.payer.to_account_info()
            },
        );
        token::mint_to(cpi_ctx,amount)?;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(decimals: u8)]
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

#[derive(Accounts)]
pub struct MintTokens<'info> {

    #[account(mut)]
    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub receiver_account:Account<'info, TokenAccount>,

    pub payer:Signer<'info>,

    pub token_program: Program<'info, Token>,
}
