use anchor_lang::prelude::*;

declare_id!("9pkx6QEi177w3mqWyyZ3cF44pVMtdsT16f5jTcNdMUr9");

#[program]
pub mod token_mover {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
