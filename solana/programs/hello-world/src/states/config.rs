pub use anchor_lang::prelude::*;

#[derive(Default, AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
pub struct WormholeAddresses {
    pub bridge: Pubkey,
    pub fee_collector: Pubkey,
    pub sequence: Pubkey,
}

#[account]
#[derive(InitSpace)]
pub struct Config {
    pub owner: Pubkey,
    pub wormhole: WormholeAddresses,
    pub batch_id: u32,
    pub finality: u8,
}

impl Config {
    pub const SEED_PREFIX: &'static [u8] = b"config";
}
