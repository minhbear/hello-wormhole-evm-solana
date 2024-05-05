import { ChainId } from "@certusone/wormhole-sdk";
import { Connection, PublicKey } from "@solana/web3.js";
import { createHelloWorldProgramInterface } from "../program";

export const deriveForeignEmitterKey = (
  programId: PublicKey,
  chain: ChainId
) => {
  return PublicKey.findProgramAddressSync(
    [
      Buffer.from("foreign_emitter"),
      (() => {
        const buf = Buffer.alloc(2);
        buf.writeUInt16LE(chain);
        return buf;
      })(),
    ],
    programId
  )[0];
};

export interface ForeignEmitter {
  chain: ChainId;
  address: Buffer;
}

export const getForeignEmitterData = async (payloads: {
  connection: Connection;
  programId: PublicKey;
  chain: ChainId;
}): Promise<ForeignEmitter> => {
  const { chain, connection, programId } = payloads;

  const { address } = await createHelloWorldProgramInterface({
    connection,
    programId,
  }).account.foreignEmitter.fetch(deriveForeignEmitterKey(programId, chain));

  return {
    chain,
    address: Buffer.from(address),
  };
};
