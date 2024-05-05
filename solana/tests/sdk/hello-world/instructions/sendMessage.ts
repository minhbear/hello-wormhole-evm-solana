import {
  Connection,
  PublicKey,
  PublicKeyInitData,
  TransactionInstruction,
} from "@solana/web3.js";
import { getPostMessageCpiAccounts } from "@certusone/wormhole-sdk/lib/cjs/solana";
import { createHelloWorldProgramInterface } from "../program";
import { deriveConfigKey, deriveWormholeMessageKey } from "../accounts";
import { getProgramSequenceTracker } from "@certusone/wormhole-sdk/lib/cjs/solana/wormhole";

export const createSendMessageInstruction = async (payloads: {
  connection: Connection;
  programId: PublicKey;
  payer: PublicKeyInitData;
  wormholeProgramId: PublicKeyInitData;
  helloMessage: Buffer;
}): Promise<TransactionInstruction> => {
  const { connection, helloMessage, payer, programId, wormholeProgramId } =
    payloads;

  const program = createHelloWorldProgramInterface({ connection, programId });  
  // get sequence
  const message = await getProgramSequenceTracker(
    connection,
    programId,
    wormholeProgramId
  ).then((tracker) => {
    return deriveWormholeMessageKey(programId, tracker.sequence + 1n);
  });
  const wormholeAccounts = getPostMessageCpiAccounts(
    programId,
    wormholeProgramId,
    payer,
    message
  );

  return await program.methods
    .sendMessage(helloMessage)
    .accounts({
      config: deriveConfigKey(programId),
      wormholeProgram: new PublicKey(wormholeProgramId),
      ...wormholeAccounts,
    })
    .instruction();
};
