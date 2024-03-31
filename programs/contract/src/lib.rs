use anchor_lang::prelude::*;

declare_id!("9EP3RUfKCHk9ERWwTKd2k2QxQAJU4agBQVeND5DSo41c");

#[program]
pub mod contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
