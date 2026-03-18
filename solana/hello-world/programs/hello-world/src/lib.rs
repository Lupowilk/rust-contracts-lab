use anchor_lang::prelude::*;

declare_id!("6wmDpCS3xyi6cxP22W4jjcSaNHBBJ5NAtECRYy8yaQGU");

#[program]
pub mod hello_world {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
