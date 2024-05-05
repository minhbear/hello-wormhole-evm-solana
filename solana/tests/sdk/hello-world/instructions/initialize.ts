import {
  Connection,
  PublicKey,
  PublicKeyInitData,
  TransactionInstruction,
} from "@solana/web3.js";
import { getPostMessageCpiAccounts } from "@certusone/wormhole-sdk/lib/cjs/solana";
import { createHelloWorldProgramInterface } from "../program";
import { deriveConfigKey, deriveWormholeMessageKey } from "../accounts";

export const createInitializeInstruction = async (payloads: {
  connection: Connection;
  programId: PublicKey;
  payer: PublicKeyInitData;
  wormholeProgramId: PublicKeyInitData;
}): Promise<TransactionInstruction> => {
  const { connection, payer, programId, wormholeProgramId } = payloads;

  const program = createHelloWorldProgramInterface({ connection, programId });
  const message = deriveWormholeMessageKey(programId, 1n);
  const wormholeAccounts = getPostMessageCpiAccounts(
    program.programId,
    wormholeProgramId,
    payer,
    message
  );

  return await program.methods
    .initialize()
    .accounts({
      owner: new PublicKey(payer),
      config: deriveConfigKey(programId),
      wormholeProgram: new PublicKey(wormholeProgramId),
      ...wormholeAccounts,
    })
    .instruction();
};
