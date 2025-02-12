// @ts-ignore-line
import readline from "readline";
import { BorshCoder, EventParser, web3 } from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Portfolio } from "../target/types/portfolio";
import { PacketPath } from "@layerzerolabs/lz-v2-utilities";
import {
  Commitment,
  Connection,
  GetAccountInfoConfig,
  PublicKey,
} from "@solana/web3.js";
import * as accounts from "./solita-generated/accounts";
import { hexlify } from "@ethersproject/bytes";
import {
  EndpointProgram,
  SimpleMessageLibProgram,
  UlnProgram,
} from "@layerzerolabs/lz-solana-sdk-v2";
export const getUserInput = (question: string): Promise<string> => {
  const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout,
  });

  return new Promise((resolve) => {
    rl.question(question, (answer) => {
      rl.close();
      resolve(answer);
    });
  });
};

export const getAccountPubKey = (
  program: Program<Portfolio>,
  seed: any[]
): web3.PublicKey => {
  let [pubkey, bump] = web3.PublicKey.findProgramAddressSync(
    seed,
    program.programId
  );

  return pubkey;
};

export const padSymbol = (symbol: string): Uint8Array => {
  const buffer = Buffer.alloc(32, 0); // Initialize a 32-byte buffer filled with zeros
  const symbolBuffer = Buffer.from(symbol);
  if (symbolBuffer.length > 32) {
    throw new Error("Symbol exceeds 32 bytes");
  }
  symbolBuffer.copy(buffer, 0); // Copy the symbol bytes to the start of the buffer
  return buffer;
};

export const printTransactionEvents = async (
  program: Program<Portfolio>,
  tx: string
) => {
  const transaction = await program.provider.connection.getTransaction(tx, {
    commitment: "confirmed",
    maxSupportedTransactionVersion: 0,
  });

  const eventParser = new EventParser(
    program.programId,
    new BorshCoder(program.idl)
  );
  const events = [
    ...eventParser.parseLogs(transaction?.meta?.logMessages ?? []),
  ];
  if (events.length > 0) {
    console.log("==Events==");
    for (let event of events) {
      console.log(event);
    }
  }
};
export const getLayerzeroSendRemainingAccounts = async (
  connection: Connection,
  srcEid: number,
  dstEid: number,
  payer: PublicKey,
  bridgePDA: PublicKey,
  remotePDA: PublicKey,
  commitmentOrConfig: Commitment | GetAccountInfoConfig = "confirmed"
) => {
  const endpoint = await getEndpoint(connection, bridgePDA);

  const msgLibProgram = await getSendLibraryProgram(
    connection,
    payer,
    dstEid,
    bridgePDA,
    endpoint
  );

  const receiverInfo = await accounts.Remote.fromAccountAddress(
    connection,
    remotePDA,
    commitmentOrConfig
  );

  const packetPath: PacketPath = {
    srcEid,
    dstEid,
    sender: hexlify(bridgePDA.toBytes()),
    receiver: hexlify(receiverInfo.address),
  };

  const ra = await endpoint.getSendIXAccountMetaForCPI(
    connection as any,
    payer,
    packetPath,
    msgLibProgram,
    commitmentOrConfig
  );
  return ra;
};

async function getEndpoint(
  connection: Connection,
  bridgePDA: PublicKey
): Promise<EndpointProgram.Endpoint> {
  const info = await accounts.Bridge.fromAccountAddress(connection, bridgePDA);
  const programAddr = info.endpointProgram;
  const endpoint = new EndpointProgram.Endpoint(programAddr);
  return endpoint;
}

async function getSendLibraryProgram(
  connection: Connection,
  payer: PublicKey,
  dstEid: number,
  bridgePDA: PublicKey,
  endpoint?: EndpointProgram.Endpoint
): Promise<SimpleMessageLibProgram.SimpleMessageLib | UlnProgram.Uln> {
  if (!endpoint) {
    endpoint = await getEndpoint(connection, bridgePDA);
  }

  const sendLibInfo = await endpoint.getSendLibrary(
    connection as any,
    bridgePDA,
    dstEid
  );

  if (!sendLibInfo?.programId) {
    throw new Error("Send library not initialized or blocked message library");
  }
  const { programId: msgLibProgram } = sendLibInfo;
  const msgLibVersion = await endpoint.getMessageLibVersion(
    connection as any,
    payer,
    msgLibProgram
  );
  if (
    msgLibVersion?.major.toString() === "0" &&
    msgLibVersion.minor == 0 &&
    msgLibVersion.endpointVersion == 2
  ) {
    return new SimpleMessageLibProgram.SimpleMessageLib(msgLibProgram);
  } else if (
    msgLibVersion?.major.toString() === "3" &&
    msgLibVersion.minor == 0 &&
    msgLibVersion.endpointVersion == 2
  ) {
    return new UlnProgram.Uln(msgLibProgram);
  }

  throw new Error(
    `Unsupported message library version: ${JSON.stringify(
      msgLibVersion,
      null,
      2
    )}`
  );
}
