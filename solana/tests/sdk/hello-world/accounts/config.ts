import { Connection, PublicKey } from "@solana/web3.js";
import { createHelloWorldProgramInterface } from "../program";

export const deriveConfigKey = (programId: PublicKey) => {
  return PublicKey.findProgramAddressSync([Buffer.from("config")], programId)[0];
}

export interface WormholeAddresses {
  bridge: PublicKey;
  feeCollector: PublicKey;
  sequence: PublicKey;
}

export interface ConfigData {
  owner: PublicKey;
  wormhole: WormholeAddresses;
}

export async function getConfigData(
  connection: Connection,
  programId: PublicKey
): Promise<ConfigData> {
  const data = await createHelloWorldProgramInterface({connection, programId})
    .account.config.fetch(deriveConfigKey(programId));

  return {
    owner: data.owner,
    wormhole: data.wormhole
  };
}
