import {
  Connection,
  PublicKey,
  PublicKeyInitData,
  TransactionInstruction,
} from "@solana/web3.js";
import { createHelloWorldProgramInterface } from "../program";
import {
  deriveConfigKey,
  deriveForeignEmitterKey,
  deriveReceivedKey,
} from "../accounts";
import {
  ChainId,
  isBytes,
  ParsedVaa,
  parseVaa,
  SignedVaa,
} from "@certusone/wormhole-sdk";
import { derivePostedVaaKey } from "@certusone/wormhole-sdk/lib/cjs/solana/wormhole";

export const createReceiveMessageInstruction = async (payloads: {
  connection: Connection;
  programId: PublicKeyInitData;
  payer: PublicKeyInitData;
  wormholeProgramId: PublicKeyInitData;
  wormholeMessage: SignedVaa | ParsedVaa;
}): Promise<TransactionInstruction> => {
  const { connection, payer, programId, wormholeMessage, wormholeProgramId } =
    payloads;

  const program = createHelloWorldProgramInterface({ connection, programId });

  const parsed = isBytes(wormholeMessage)
    ? parseVaa(wormholeMessage)
    : wormholeMessage;

  return await program.methods
    .receiveMessage([...parsed.hash])
    .accounts({
      payer: new PublicKey(payer),
      config: deriveConfigKey(programId),
      wormholeProgram: new PublicKey(wormholeProgramId),
      posted: derivePostedVaaKey(wormholeProgramId, parsed.hash),
      foreignEmitter: deriveForeignEmitterKey(
        programId,
        parsed.emitterChain as ChainId
      ),
      received: deriveReceivedKey({
        programId,
        chain: parsed.emitterChain as ChainId,
        sequence: parsed.sequence,
      }),
    })
    .instruction();
};
