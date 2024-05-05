use anchor_lang::prelude::*;

pub const MESSAGE_MAX_LENGTH: usize = 1024;

#[account]
#[derive(Default, InitSpace)]
pub struct Received {
    pub batch_id: u32,
    /// Keccak256 hash of verified Wormhole message.
    pub wormhole_message_hash: [u8; 32],
    /// HelloWorldMessage from [HelloWorldMessage::Hello](crate::message::HelloWorldMessage).
    #[max_len(MESSAGE_MAX_LENGTH)]
    pub message: Vec<u8>,
}

impl Received {
    pub const SEED_PREFIX: &'static [u8; 8] = b"received";
}