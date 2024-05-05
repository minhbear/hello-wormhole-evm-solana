import { PublicKey, Keypair } from "@solana/web3.js";
import { CONTRACTS } from "@certusone/wormhole-sdk";
import { MockGuardians } from "@certusone/wormhole-sdk/lib/cjs/mock";

export const NETWORK = "TESTNET";

export const WORMHOLE_CONTRACTS = CONTRACTS[NETWORK];
export const CORE_BRIDGE_PID = new PublicKey(WORMHOLE_CONTRACTS.solana.core);

export const TOKEN_BRIDGE_PID = new PublicKey(
  WORMHOLE_CONTRACTS.solana.token_bridge
);

export const LOCALHOST = "http://localhost:8899";
export const DEV_NET = "https://api.devnet.solana.com";

export const PAYER_KEYPAIR = Keypair.fromSecretKey(
  Uint8Array.from([
    7, 193, 77, 237, 177, 134, 82, 130, 43, 211, 204, 193, 217, 183, 228, 60,
    194, 167, 70, 28, 14, 8, 92, 105, 51, 226, 62, 93, 207, 124, 189, 35, 77,
    32, 61, 182, 77, 14, 63, 225, 142, 76, 42, 55, 253, 226, 121, 194, 8, 28,
    147, 80, 19, 239, 78, 242, 158, 97, 153, 154, 49, 200, 151, 88,
  ])
);
export const RELAYER_KEYPAIR = Keypair.fromSecretKey(
  Uint8Array.from([
    209, 193, 148, 98, 190, 29, 112, 141, 167, 133, 181, 253, 103, 0, 148, 205,
    111, 214, 146, 194, 94, 126, 194, 28, 188, 221, 72, 105, 190, 41, 91, 39,
    237, 124, 31, 221, 91, 218, 22, 33, 230, 41, 14, 203, 176, 164, 200, 245,
    31, 19, 161, 61, 30, 188, 11, 120, 155, 236, 178, 241, 114, 240, 67, 3,
  ])
);

export const MOCK_GUARDIANS = new MockGuardians(0, [
  "cfb12303a19cde580bb4dd771639b0d26bc68353645571a8cff516ab2ee113a0",
]);
