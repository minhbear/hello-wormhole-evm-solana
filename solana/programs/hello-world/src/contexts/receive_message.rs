use anchor_lang::prelude::*;
use wormhole_anchor_sdk::wormhole::{program::Wormhole, PostedVaa, SEED_PREFIX_POSTED_VAA};

use crate::{
    common::HelloWorldError, Config, ForeignEmitter, HelloWorldMessage, Received,
    MESSAGE_MAX_LENGTH,
};

#[derive(Accounts)]
#[instruction(vaa_hash: [u8; 32])]
pub struct ReceiveMessage<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        seeds = [Config::SEED_PREFIX],
        bump,
    )]
    pub config: Account<'info, Config>,

    pub wormhole_program: Program<'info, Wormhole>,

    #[account(
        seeds = [
            SEED_PREFIX_POSTED_VAA,
            &vaa_hash
        ],
        bump,
        seeds::program = wormhole_program
    )]
    pub posted: Account<'info, PostedVaa<HelloWorldMessage>>,

    #[account(
        seeds = [
            ForeignEmitter::SEED_PREFIX,
            &posted.emitter_chain().to_le_bytes()[..]
        ],
        bump,
        constraint = foreign_emitter.verify(posted.emitter_address()) @ HelloWorldError::InvalidForeignEmitter
    )]
    pub foreign_emitter: Account<'info, ForeignEmitter>,

    #[account(
        init,
        payer = payer,
        seeds = [
            Received::SEED_PREFIX,
            &posted.emitter_chain().to_le_bytes()[..],
            &posted.sequence().to_le_bytes()[..]
        ],
        bump,
        space = Received::INIT_SPACE
    )]
    pub received: Account<'info, Received>,

    pub system_program: Program<'info, System>,
}

impl<'info> ReceiveMessage<'info> {
    pub fn receive_message(&mut self, vaa_hash: [u8; 32]) -> Result<()> {
        let posted_message = &self.posted;

        if let HelloWorldMessage::Hello { message } = posted_message.data() {
            // HelloWorldMessage cannt be larger than the maximum size of the account
            require!(
                message.len() <= MESSAGE_MAX_LENGTH,
                HelloWorldError::InvalidMessage
            );

            // Save batch ID, keccak256 hash and message payload
            let received = &mut self.received;
            received.batch_id = posted_message.batch_id();
            received.wormhole_message_hash = vaa_hash;
            received.message = message.clone();

            Ok(())
        } else {
            err!(HelloWorldError::InvalidMessage)
        }
    }
}
