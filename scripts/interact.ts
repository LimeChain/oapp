import { Connection, PublicKey } from "@solana/web3.js";
import { Options } from "@layerzerolabs/lz-v2-utilities";
import { sendAndConfirm } from "./util";
import { connection, signer, bridgeProgram } from "./common";
import { EndpointId } from "@layerzerolabs/lz-definitions";
import { accounts } from "../generated/bridge";

(async () => {
  await callSend(connection, signer.publicKey, 40267);
  //generateOptions();
  //await getId(connection);
  // await getPeer(connection, 40267);
})();

async function callSend(
  connection: Connection,
  payer: PublicKey,
  dstEid: number
) {
  const ix = await bridgeProgram.send(connection, payer, dstEid);
  await sendAndConfirm(connection, [signer], [ix]);
}

async function getPeer(
  connection: Connection,
  remote: EndpointId
): Promise<void> {
  const [remotePDA] = bridgeProgram.deriver.remote(remote);
  try {
    const info = await accounts.Remote.fromAccountAddress(
      connection,
      remotePDA,
      {
        commitment: "confirmed",
      }
    );
    const peer = "0x" + Buffer.from(info.address).toString("hex");
    console.log(peer);
  } catch (e) {
    // remote not initialized
    console.log(e);
  }
}

async function getSendLib(
  connection: Connection,
  payer: PublicKey,
  destId: number
) {
  const sendLib = await bridgeProgram.getSendLibraryProgram(
    connection,
    payer,
    destId
  );
  console.log(`sendLib: ${sendLib.program}`);
}

async function getId(connection: Connection) {
  const [id] = bridgeProgram.idPDA();
  console.log(id);
}

function generateOptions() {
  const options = Options.newOptions()
    .addExecutorLzReceiveOption(300000, 0)
    .toHex();
  console.log(options);
}
