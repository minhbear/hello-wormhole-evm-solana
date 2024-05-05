import { deriveWormholeEmitterKey } from "@certusone/wormhole-sdk/lib/cjs/solana/wormhole";
import { Connection, PublicKey } from "@solana/web3.js";
import { createHelloWorldProgramInterface } from "../program";

export { deriveWormholeEmitterKey };

export const deriveWormholeMessageKey = (
  programId: PublicKey,
  sequence: bigint
) => {
  return PublicKey.findProgramAddressSync(
    [
      Buffer.from("sent"),
      (() => {
        const buf = Buffer.alloc(8);
        buf.writeBigUInt64LE(sequence);
        return buf;
      })(),
    ],
    programId
  )[0];
}

export interface WormholeEmitterData {
  bump: number;
}

export const getWormholeEmitterData = async (
  connection: Connection,
  programId: PublicKey
): Promise<WormholeEmitterData> => {
  return createHelloWorldProgramInterface({connection, programId})
    .account.wormholeEmitter.fetch(deriveWormholeEmitterKey(programId));
}
