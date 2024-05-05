pub use anchor_lang::prelude::*;
use wormhole_anchor_sdk::wormhole::CHAIN_ID_SOLANA;

use crate::{Config, ForeignEmitter, common::HelloWorldError};

#[derive(Accounts)]
#[instruction(chain: u16)]
pub struct RegisterEmitter<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
      has_one = owner @ HelloWorldError::OwnerOnly,
      seeds = [Config::SEED_PREFIX],
      bump
    )]
    pub config: Account<'info, Config>,

    #[account(
      init_if_needed,
      payer = owner,
      space = ForeignEmitter::INIT_SPACE + 8,
      seeds = [
        ForeignEmitter::SEED_PREFIX,
        &chain.to_le_bytes()[..]
      ],
      bump
    )]
    /// Foreign Emitter account. Create this account if an emitter has not been
    /// registered yet for this Wormhole chain ID. If there already is an
    /// emitter address saved in this account, overwrite it.
    pub foreign_emitter: Account<'info, ForeignEmitter>,

    pub system_program: Program<'info, System>
}

impl<'info> RegisterEmitter<'info> {
  pub fn register_emitter(&mut self, chain: u16, address: [u8; 32]) -> Result<()> {
    // Foreign emitter cannot share the same Wormhole Chain ID as the
    // Solana Wormhole program's. And cannot register a zero address.
    require!(
      chain > 0 && chain != CHAIN_ID_SOLANA && !address.iter().all(|&x| x == 0),
      HelloWorldError::InvalidForeignEmitter
    );

    // Save the emitter info into the ForeignEmitter account
    let emitter = &mut self.foreign_emitter;
    emitter.chain = chain;
    emitter.address = address;

    Ok(())
  }
}