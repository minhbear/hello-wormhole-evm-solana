pub mod common;
pub mod contexts;
pub mod message;
pub mod states;

pub use contexts::*;
pub use message::*;
pub use states::*;

declare_id!("2NV1iSwqoG8iKpCSQEyG2e2je46djm8jr9KqYnLgwt5z");

#[program]
pub mod hello_world {
    use super::*;

    /// This instruction initializes the program config, which is meant
    /// to store data useful for other instructions. The config specifies
    /// an owner (e.g. multisig) and should be read-only for every instruction
    /// in this example. This owner will be checked for designated owner-only
    /// instructions like [`register_emitter`](register_emitter).
    ///
    /// # Arguments
    ///
    /// * `ctx` - `Initialize` context
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps)
    }

    /// This instruction registers a new foreign emitter (from another network)
    /// and saves the emitter information in a ForeignEmitter account. This
    /// instruction is owner-only, meaning that only the owner of the program
    /// (defined in the [Config] account) can add and update emitters.
    ///
    /// # Arguments
    ///
    /// * `ctx`     - `RegisterForeignEmitter` context
    /// * `chain`   - Wormhole Chain ID
    /// * `address` - Wormhole Emitter Address
    pub fn register_emitter(
        ctx: Context<RegisterEmitter>,
        chain: u16,
        address: [u8; 32],
    ) -> Result<()> {
        ctx.accounts.register_emitter(chain, address)
    }

    /// This instruction posts a Wormhole message of some arbitrary size
    /// in the form of bytes ([Vec<u8>]). The message is encoded as
    /// [HelloWorldMessage::Hello], which serializes a payload ID (1) before the message
    /// specified in the instruction. Instead of using the native borsh
    /// serialization of [Vec] length (little endian u32), length of the
    /// message is encoded as big endian u16 (in EVM, bytes for numerics are
    /// natively serialized as big endian).
    ///
    /// See [HelloWorldMessage] enum for serialization implementation.
    ///
    /// # Arguments
    ///
    /// * `message` - Arbitrary message to send out
    pub fn send_message(ctx: Context<SendMessage>, message: Vec<u8>) -> Result<()> {
        ctx.accounts.send_message(&ctx.bumps, message)
    }

    /// This instruction reads a posted verified Wormhole message and verifies
    /// that the payload is of type [HelloWorldMessage::Hello] (payload ID == 1). HelloWorldMessage
    /// data is stored in a [Received] account.
    ///
    /// See [HelloWorldMessage] enum for deserialization implementation.
    ///
    /// # Arguments
    ///
    /// * `vaa_hash` - Keccak256 hash of verified Wormhole message
    pub fn receive_message(ctx: Context<ReceiveMessage>, vaa_hash: [u8; 32]) -> Result<()> {
        ctx.accounts.receive_message(vaa_hash)
    }
}
