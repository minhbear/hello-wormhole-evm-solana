use anchor_lang::prelude::*;
use wormhole_anchor_sdk::wormhole::{
    post_message, program::Wormhole, BridgeData, FeeCollector, Finality, PostMessage,
    SequenceTracker, INITIAL_SEQUENCE, SEED_PREFIX_EMITTER,
};

use crate::{program::HelloWorld, states::Config, HelloWorldMessage, states::WormholeEmitter};

pub const SEED_PREFIX_SENT: &[u8; 4] = b"sent";

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
      init,
      payer = owner,
      seeds = [Config::SEED_PREFIX],
      bump,
      space = Config::INIT_SPACE
    )]
    pub config: Account<'info, Config>,

    pub wormhole_program: Program<'info, Wormhole>,

    #[account(
      mut,
      seeds = [BridgeData::SEED_PREFIX],
      bump,
      seeds::program = wormhole_program
    )]
    pub wormhole_bridge: Account<'info, BridgeData>,

    #[account(
      mut,
      seeds = [FeeCollector::SEED_PREFIX],
      bump,
      seeds::program = wormhole_program
    )]
    pub wormhole_fee_collector: Account<'info, FeeCollector>,

    #[account(
      init,
      payer = owner,
      seeds = [WormholeEmitter::SEED_PREFIX],
      bump,
      space = WormholeEmitter::INIT_SPACE
    )]
    pub wormhole_emitter: Account<'info, WormholeEmitter>,

    #[account(
      mut,
      seeds = [
        SequenceTracker::SEED_PREFIX,
        wormhole_emitter.key().as_ref()
      ],
      bump,
      seeds::program = wormhole_program
    )]
    /// CHECK: Emitter's sequence account. This is not created until the first
    /// message is posted
    pub wormhole_sequence: UncheckedAccount<'info>,

    #[account(
      mut,
      seeds = [
          SEED_PREFIX_SENT,
          &INITIAL_SEQUENCE.to_le_bytes()[..]
      ],
      bump,
    )]
    /// CHECK: Wormhole message account. The Wormhole program writes to this
    /// account, which requires this program's signature.
    /// [`wormhole::post_message`] requires this account be mutable.
    pub wormhole_message: UncheckedAccount<'info>,

    pub clock: Sysvar<'info, Clock>,

    pub rent: Sysvar<'info, Rent>,

    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, bumps: &InitializeBumps) -> Result<()> {
        let config = &mut self.config;

        // Set the owner of the config (effectively the owner of the program)
        config.owner = self.owner.key();

        // Set wormhole related addresses
        {
            let wormhole = &mut config.wormhole;

            // wormhole::BridgeData (Wormhole's program data)
            wormhole.bridge = self.wormhole_bridge.key();

            // wormhole::FeeCollector (lamports collector for posting
            // messages).
            wormhole.fee_collector = self.wormhole_fee_collector.key();

            // wormhole::SequenceTracker (tracks # of messages posted by this program)
            wormhole.sequence = self.wormhole_sequence.key();
        }

        // Set default values for posting Wormhole messages.
        // 0 means no batching
        config.batch_id = 0;

        // Anchor IDL default coder cannot handle wormhole::Finality enum,
        // so this value is stored as u8
        config.finality = Finality::Confirmed as u8;

        // Initialize our Wormhole emitter account. It is not required by the
        // Wormhole program that there is an actual account associated with the
        // emitter PDA. The emitter PDA is just a mechanism to have the program
        // sign for the `wormhole::post_message` instruction.
        //
        // But for fun, we will store our emitter's bump for convenience.
        self.wormhole_emitter.bump = bumps.wormhole_emitter;

        // This scope shows the steps of how to post a message with the Wormhole program
        {
            // If Wormhole requires a fee before posting a message, we need to
            // transfer lamports to the fee collector. Otherwise
            // `wormhole::post_message` will fail.
            let fee = self.wormhole_bridge.fee();
            if fee > 0 {
                solana_program::program::invoke(
                    &solana_program::system_instruction::transfer(
                        &self.owner.key,
                        &self.wormhole_fee_collector.key(),
                        fee,
                    ),
                    &self.to_account_infos(),
                )?;
            }

            // Invoke `wormhole::post_message`. We are sending a Wormhole
            // message in the `initialize` instruction so the Wormhole program
            // can create a SequenceTracker account for our emitter. We will
            // deserialize this account for our `send_message` instruction so
            // we can find the next sequence number. More details about this in
            // `send_message`.
            //
            // `wormhole::post_message` requires two signers: one for the
            // emitter and another for the wormhole message data. Both of these
            // accounts are owned by this program.
            //
            // There are two ways to handle the wormhole message data account:
            //   1. Using an extra keypair. You may to generate a keypair
            //      outside of this instruction and pass that keypair as an
            //      additional signer for the transaction. An integrator might
            //      use an extra keypair if the message can be "thrown away"
            //      (not easily retrievable without going back to this
            //      transaction hash to retrieve the message's pubkey).
            //   2. Generate a PDA. If we want some way to deserialize the
            //      message data written by the Wormhole program, we can use an
            //      account with an address derived by this program so we can
            //      use the PDA to access and deserialize the message data.
            //
            // In our example, we use method #2.
            let wormhole_emitter = &self.wormhole_emitter;
            let config = &self.config;

            // If anyone were to care about the first message this program emits,
            // he can deserialize it to find the program with which the emitter PDA
            // was derived
            let mut payload: Vec<u8> = Vec::new();
            HelloWorldMessage::serialize(
                &HelloWorldMessage::Alive {
                    program_id: HelloWorld::id(),
                },
                &mut payload,
            )?;

            post_message(
                CpiContext::new_with_signer(
                    self.wormhole_program.to_account_info(),
                    PostMessage {
                        config: self.wormhole_bridge.to_account_info(),
                        message: self.wormhole_message.to_account_info(),
                        emitter: self.wormhole_emitter.to_account_info(),
                        sequence: self.wormhole_sequence.to_account_info(),
                        payer: self.owner.to_account_info(),
                        fee_collector: self.wormhole_fee_collector.to_account_info(),
                        clock: self.clock.to_account_info(),
                        rent: self.rent.to_account_info(),
                        system_program: self.system_program.to_account_info(),
                    },
                    &[
                        &[
                            SEED_PREFIX_SENT,
                            &INITIAL_SEQUENCE.to_le_bytes()[..],
                            &[bumps.wormhole_message],
                        ],
                        &[SEED_PREFIX_EMITTER, &[wormhole_emitter.bump]],
                    ],
                ),
                config.batch_id,
                payload,
                config.finality.try_into().unwrap(),
            )?;
        }

        Ok(())
    }
}
