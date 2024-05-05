pub use anchor_lang::prelude::*;

#[account]
#[derive(Debug, InitSpace, Default)]
pub struct WormholeAddresses {
  /// [BridgeData](wormhole_anchor_sdk::wormhole::BridgeData) address.
  pub bridge: Pubkey,
  /// [FeeCollector](wormhole_anchor_sdk::wormhole::FeeCollector) address.
  pub fee_collector: Pubkey,
  /// [SequenceTracker](wormhole_anchor_sdk::wormhole::SequenceTracker) address.
  pub sequence: Pubkey,
}

#[account]
#[derive(InitSpace, Debug, Default)]
pub struct Config {
  pub owner: Pubkey,
  pub wormhole: WormholeAddresses,
  pub batch_id: u32,
  pub finality: u8
}

impl Config {
  pub const SEED_PREFIX: &'static [u8; 6] = b"config";
}