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
