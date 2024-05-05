use anchor_lang::prelude::*;
use wormhole_anchor_sdk::wormhole::{
    post_message, program::Wormhole, BridgeData, FeeCollector, PostMessage, SequenceTracker,
    SEED_PREFIX_EMITTER,
};

use crate::{
    common::HelloWorldError, Config, HelloWorldMessage, WormholeEmitter, SEED_PREFIX_SENT,
};

#[derive(Accounts)]
pub struct SendMessage<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
      seeds = [Config::SEED_PREFIX],
      bump,
    )]
    pub config: Account<'info, Config>,

    pub wormhole_program: Program<'info, Wormhole>,

    #[account(
      mut,
      address = config.wormhole.bridge @ HelloWorldError::InvalidWormholeConfig
    )]
    pub wormhole_bridge: Account<'info, BridgeData>,

    #[account(
      mut,
      address = config.wormhole.fee_collector @ HelloWorldError::InvalidWormholeFeeCollector
    )]
    pub wormhole_fee_collector: Account<'info, FeeCollector>,

    #[account(
      seeds = [WormholeEmitter::SEED_PREFIX],
      bump,
    )]
    pub wormhole_emitter: Account<'info, WormholeEmitter>,

    #[account(
      mut,
      address = config.wormhole.sequence @ HelloWorldError::InvalidWormholeSequence
    )]
    pub wormhole_sequence: Account<'info, SequenceTracker>,

    #[account(
      mut,
      seeds = [
          SEED_PREFIX_SENT,
          &wormhole_sequence.next_value().to_le_bytes()[..]
      ],
      bump,
    )]
    pub wormhole_message: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,

    pub clock: Sysvar<'info, Clock>,

    pub rent: Sysvar<'info, Rent>,
}

impl<'info> SendMessage<'info> {
    pub fn send_message(&mut self, bumps: &SendMessageBumps, message: Vec<u8>) -> Result<()> {
        // If Wormhole requires a fee before posting a message, we need to
        // transfer lamports to the fee collector. Otherwise
        // `wormhole::post_message` will fail.
        let fee = self.wormhole_bridge.fee();
        if fee > 0 {
            solana_program::program::invoke(
                &solana_program::system_instruction::transfer(
                    &self.payer.key(),
                    &self.wormhole_fee_collector.key(),
                    fee,
                ),
                &self.to_account_infos(),
            )?;
        }

        // Invoke `wormhole::post_message`.
        //
        // `wormhole::post_message` requires two signers: one for the emitter
        // and another for the wormhole message data. Both of these accounts
        // are owned by this program.
        //
        // There are two ways to handle the wormhole message data account:
        //   1. Using an extra keypair. You may to generate a keypair outside
        //      of this instruction and pass that keypair as an additional
        //      signer for the transaction. An integrator might use an extra
        //      keypair if the message can be "thrown away" (not easily
        //      retrievable without going back to this transaction hash to
        //      retrieve the message's pubkey).
        //   2. Generate a PDA. If we want some way to deserialize the message
        //      data written by the Wormhole program, we can use an account
        //      with an address derived by this program so we can use the PDA
        //      to access and deserialize the message data.
        //
        // In our example, we use method #2.
        let config = &self.config;

        // There is only one type of message that this example uses to
        // communicate with its foreign counterparts (payload ID == 1).
        let payload: Vec<u8> = HelloWorldMessage::Hello { message }.try_to_vec()?;

        post_message(
            CpiContext::new_with_signer(
                self.wormhole_program.to_account_info(),
                PostMessage {
                    config: self.wormhole_bridge.to_account_info(),
                    message: self.wormhole_message.to_account_info(),
                    emitter: self.wormhole_emitter.to_account_info(),
                    sequence: self.wormhole_sequence.to_account_info(),
                    payer: self.payer.to_account_info(),
                    fee_collector: self.wormhole_fee_collector.to_account_info(),
                    clock: self.clock.to_account_info(),
                    rent: self.rent.to_account_info(),
                    system_program: self.system_program.to_account_info(),
                },
                &[
                    &[
                        SEED_PREFIX_SENT,
                        &self.wormhole_sequence.next_value().to_le_bytes()[..],
                        &[bumps.wormhole_message],
                    ],
                    &[SEED_PREFIX_EMITTER, &[bumps.wormhole_emitter]],
                ],
            ),
            config.batch_id,
            payload,
            config.finality.try_into().unwrap(),
        )?;

        Ok(())
    }
}
