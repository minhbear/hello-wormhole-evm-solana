import { expect, use as chaiUse } from "chai";
import chaiAsPromised from "chai-as-promised";
chaiUse(chaiAsPromised);
import {
  Connection,
  Keypair,
  PublicKey,
  Ed25519Program,
} from "@solana/web3.js";
import { CHAINS, ChainId, parseVaa } from "@certusone/wormhole-sdk";
import * as mock from "@certusone/wormhole-sdk/lib/cjs/mock";
import { getPostMessageCpiAccounts } from "@certusone/wormhole-sdk/lib/cjs/solana";
import * as wormhole from "@certusone/wormhole-sdk/lib/cjs/solana/wormhole";
import * as helloWorld from "./sdk/hello-world";
import * as anchor from "@coral-xyz/anchor";
import {
  DEV_NET,
  PAYER_KEYPAIR,
  CORE_BRIDGE_PID,
  range,
  programIdFromEnvVar,
  boilerPlateReduction,
} from "./helpers";

const HELLO_WORLD_PID = new PublicKey(
  "2NV1iSwqoG8iKpCSQEyG2e2je46djm8jr9KqYnLgwt5z"
);

describe("Hello World", () => {
  const connection = new Connection(DEV_NET, "finalized");
  const payer = PAYER_KEYPAIR;

  const {
    requestAirdrop,
    guardianSign,
    postSignedMsgAsVaaOnSolana,
    expectIxToSucceed,
    expectIxToFailWithError,
  } = boilerPlateReduction(connection, payer);

  // Set provider, connection and program
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider();
  // @ts-ignore
  const providerWallet = provider.wallet.payer as Keypair;

  // foreign emitter info
  const realForeignEmitterChain = CHAINS.klaytn;
  const realForeignEmitterAddress = Buffer.alloc(32, "deadbeef", "hex");

  const realConfig = helloWorld.deriveConfigKey(HELLO_WORLD_PID);
  const realForeignEmitter = helloWorld.deriveForeignEmitterKey(
    HELLO_WORLD_PID,
    realForeignEmitterChain
  );

  xdescribe("Initialize Program", () => {
    const createInitializeIx = () =>
      helloWorld.createInitializeInstruction({
        connection,
        programId: HELLO_WORLD_PID,
        payer: payer.publicKey,
        wormholeProgramId: CORE_BRIDGE_PID,
      });

    it("Finally Set Up Program", async function () {
      // await expectIxToSucceed(createInitializeIx(), payer);

      // verify account data
      const configData = await helloWorld.getConfigData(
        connection,
        HELLO_WORLD_PID
      );
      expect(configData.owner).deep.equals(payer.publicKey);

      const { wormholeBridge, wormholeFeeCollector } =
        wormhole.getWormholeDerivedAccounts(HELLO_WORLD_PID, CORE_BRIDGE_PID);
      expect(configData.wormhole.bridge).deep.equals(wormholeBridge);
      expect(configData.wormhole.feeCollector).deep.equals(
        wormholeFeeCollector
      );
    });

    it("Cannot Call Instruction Again: initialize", async function () {
      await expectIxToFailWithError(
        await createInitializeIx(),
        "already in use"
      );
    });
  });

  xdescribe("Register Foreign Emitter", () => {
    it("Register success Foreign Emitter", async () => {
      await expectIxToSucceed(
        helloWorld.createRegisterForeignEmitterInstruction({
          connection,
          programId: HELLO_WORLD_PID,
          payer: payer.publicKey,
          emitterChain: realForeignEmitterChain,
          emitterAddress: realForeignEmitterAddress,
        })
      );

      const { chain, address } = await helloWorld.getForeignEmitterData({
        connection,
        programId: HELLO_WORLD_PID,
        chain: realForeignEmitterChain,
      });
      expect(chain).equals(realForeignEmitterChain);
      expect(address).deep.equals(realForeignEmitterAddress);
    });
  });

  describe("Send Message", () => {
    it("Send Message Successfully", async function () {
      const helloMessage = Buffer.from("All your base are belong to us");

      // save message count to grab posted message later
      const sequence =
        (
          await wormhole.getProgramSequenceTracker(
            connection,
            HELLO_WORLD_PID,
            CORE_BRIDGE_PID,
            'finalized'
          )
        ).value() + 1n;
      await expectIxToSucceed(
        helloWorld.createSendMessageInstruction({
          connection,
          programId: HELLO_WORLD_PID,
          payer: payer.publicKey,
          wormholeProgramId: CORE_BRIDGE_PID,
          helloMessage,
        })
      );

      const { payload } = (
        await wormhole.getPostedMessage(
          connection,
          helloWorld.deriveWormholeMessageKey(HELLO_WORLD_PID, sequence)
        )
      ).message;

      expect(payload.readUint8(0)).equals(1); // payload ID
      expect(payload.readUint16BE(1)).equals(helloMessage.length);
      expect(payload.subarray(3)).deep.equals(helloMessage);
    });
  });
});
