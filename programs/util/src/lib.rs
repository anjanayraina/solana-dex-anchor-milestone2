use anchor_lang::prelude::*;

declare_id!("HCPtxSR4y8BUCeVZkFn8XGj73THg39uEmHf5h7hoaTST");

#[program]
pub mod util {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
