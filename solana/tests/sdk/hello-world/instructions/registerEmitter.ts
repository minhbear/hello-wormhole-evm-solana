import {
  Connection,
  PublicKey,
  PublicKeyInitData,
  TransactionInstruction,
} from "@solana/web3.js";
import { ChainId } from "@certusone/wormhole-sdk";
import { createHelloWorldProgramInterface } from "../program";
import { deriveConfigKey, deriveForeignEmitterKey } from "../accounts";

export const createRegisterForeignEmitterInstruction = async (payloads: {
  connection: Connection;
  programId: PublicKeyInitData;
  payer: PublicKeyInitData;
  emitterChain: ChainId;
  emitterAddress: Buffer;
}): Promise<TransactionInstruction> => {
  const { connection, emitterAddress, emitterChain, payer, programId } = payloads;

  const program = createHelloWorldProgramInterface({connection, programId});
  return await program.methods
    .registerEmitter(emitterChain, [...emitterAddress])
    .accounts({
      owner: new PublicKey(payer),
      config: deriveConfigKey(program.programId),
      foreignEmitter: deriveForeignEmitterKey(program.programId, emitterChain),
    })
    .instruction();
};
