use anchor_lang::prelude::*;

declare_id!("FQExbwU6c7DTUmMSmvHhiBw7zHiVWhEr1VR5zhHWbzCi");

#[program]
pub mod solana_dex_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
