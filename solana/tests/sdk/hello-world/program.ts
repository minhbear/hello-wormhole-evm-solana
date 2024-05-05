import { Connection, PublicKeyInitData, PublicKey } from "@solana/web3.js";
import { Program, Provider } from "@coral-xyz/anchor";

import { HelloWorld } from "../../../target/types/hello_world";
import IDL from "../../../target/idl/hello_world.json";

export const createHelloWorldProgramInterface = (payloads: {
  connection: Connection;
  programId: PublicKeyInitData;
  payer?: PublicKeyInitData;
}): Program<HelloWorld> => {
  const { connection, programId, payer } = payloads;

  const provider: Provider = {
    connection,
    publicKey: payer == undefined ? undefined : new PublicKey(payer),
  };
  
  return new Program<HelloWorld>(
    IDL as any,
    new PublicKey(programId),
    provider
  );
};
