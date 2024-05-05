use anchor_lang::prelude::*;
use wormhole_anchor_sdk::wormhole::SEED_PREFIX_EMITTER;

#[account]
#[derive(Default, InitSpace)]
pub struct WormholeEmitter {
  pub bump: u8
}

impl WormholeEmitter {
  pub const SEED_PREFIX: &'static [u8; 7] = SEED_PREFIX_EMITTER; 
}