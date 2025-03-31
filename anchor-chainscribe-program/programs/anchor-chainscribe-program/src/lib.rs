use anchor_lang::prelude::*;

mod constants;
mod contexts;
mod errors;
mod instructions;
mod state;

use contexts::*;
use instructions::*;

declare_id!("FnAbLTmMMUA6XvadsrFm8pHtAYYNHEnKo4Q7tcG5vhiL");

#[program]
pub mod anchor_chainscribe_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
