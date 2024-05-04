use anchor_lang::prelude::*;

declare_id!("Evhd16H7Qkcbh3tWP9dyBSLMxBKGVyM9QQPM83xZJEaa");

#[program]
pub mod solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
