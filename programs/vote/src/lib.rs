use anchor_lang::prelude::*;
mod state;

declare_id!("2Z1wEDNgMM9yzTSFvRULyisQJfVtMnbhpqT1P2AD3DSR");

#[program]
pub mod vote {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
