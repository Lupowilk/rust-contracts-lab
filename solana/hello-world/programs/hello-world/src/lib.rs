use anchor_lang::prelude::*; //import the Anchor SDK prelude into scope

declare_id!("6wmDpCS3xyi6cxP22W4jjcSaNHBBJ5NAtECRYy8yaQGU"); //declares the program's on-chain address

#[program] // enabkes public calls of the functions inside the mod
pub mod hello_world {
    use super::*; // enable the access to Anchor's types and macros

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
