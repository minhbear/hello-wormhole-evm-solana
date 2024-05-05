import { ChainId } from "@certusone/wormhole-sdk";
import { Connection, PublicKey } from "@solana/web3.js";
import { createHelloWorldProgramInterface } from "../program";

export const deriveReceivedKey = (payloads: {
  programId: PublicKey;
  chain: ChainId;
  sequence: bigint;
}) => {
  const { chain, programId, sequence } = payloads;

  return PublicKey.findProgramAddressSync(
    [
      Buffer.from("received"),
      (() => {
        const buf = Buffer.alloc(10);
        buf.writeUInt16LE(chain, 0);
        buf.writeBigInt64LE(sequence, 2);
        return buf;
      })(),
    ],
    programId
  )[0];
};

export interface Received {
  batchId: number;
  message: Buffer;
}

export const getReceivedData = async (payloads: {
  connection: Connection;
  programId: PublicKey;
  chain: ChainId;
  sequence: bigint;
}): Promise<Received> => {
  const { chain, connection, programId, sequence } = payloads;

  const received = await createHelloWorldProgramInterface({
    connection,
    programId,
  }).account.received.fetch(deriveReceivedKey({ programId, chain, sequence }));

  return {
    batchId: received.batchId,
    message: received.message as Buffer,
  };
};
