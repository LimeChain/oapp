import {
  EndpointProgram,
  EventPDADeriver,
  MessageType,
  SimpleMessageLibProgram,
  UlnProgram,
} from "@layerzerolabs/lz-solana-sdk-v2";
import { PDADeriver } from "./pda-deriver";
import {
  Commitment,
  Connection,
  PublicKey,
  GetAccountInfoConfig,
  TransactionInstruction,
  ComputeBudgetProgram,
  VersionedTransaction,
  TransactionMessage,
  AccountMeta,
} from "@solana/web3.js";
import * as instructions from "./instructions";
import * as types from "./types";
import * as accounts from "./accounts";
import { hexlify } from "@ethersproject/bytes";
import { Options, PacketPath } from "@layerzerolabs/lz-v2-utilities";
import { signer } from "../scripts/common";
import { BN } from "bn.js";

export { instructions, types, accounts };

// pub nonce: u64,
// pub transaction: Tx, // Tx type, mapped to a 16-bit integer
// pub trader: [u8; 32],
// pub symbol: [u8; 32],
// pub quantity: u64,
// pub timestamp: i64,
// pub customdata: [u8; 18],

const xferMessage = {
  nonce: new BN(0),
  transaction: 1,
  trader: Array(20).fill(0),
  symbol: Array(32).fill(0),
  quantity: [new BN(1), new BN(1), new BN(1), new BN(1)],
  timestamp: Math.floor(Date.now() / 1000), // if it's not in seconds in overflows
  customdata: Array(28).fill(0),
  messageType: 0,
} satisfies types.XFER;

export class Bridge {
  deriver: PDADeriver;
  endpoint: EndpointProgram.Endpoint | undefined;

  constructor(public readonly program: PublicKey) {
    this.deriver = new PDADeriver(program);
  }

  idPDA(): [PublicKey, number] {
    return this.deriver.bridge();
  }

  async initBridge(
    connection: Connection,
    payer: PublicKey,
    endpoint: EndpointProgram.Endpoint,
    commitmentOrConfig: Commitment | GetAccountInfoConfig = "confirmed"
  ) {
    const [id] = this.idPDA();
    const [oAppRegistry] = endpoint.deriver.oappRegistry(id);
    const info = await connection.getAccountInfo(id, commitmentOrConfig);
    if (info) {
      // no need to init
      return null;
    }
    const [eventAuthority] = new EventPDADeriver(
      endpoint.program
    ).eventAuthority();

    const ixAccounts =
      EndpointProgram.instructions.createRegisterOappInstructionAccounts(
        {
          payer: payer,
          oapp: id,
          oappRegistry: oAppRegistry,
          eventAuthority,
          program: endpoint.program,
        },
        endpoint.program
      );
    // these accounts are used for the CPI, so we need to set them to false
    const registerOAppAccounts = [
      {
        pubkey: endpoint.program,
        isSigner: false,
        isWritable: false,
      },
      ...ixAccounts,
    ];
    // the first two accounts are both signers, so we need to set them to false, solana will set them to signer internally
    registerOAppAccounts[1].isSigner = false;
    registerOAppAccounts[2].isSigner = false;

    return instructions.createInitBridgeInstruction(
      {
        authority: payer,
        bridge: id,
        solVault: this.deriver.solVault()[0],
        anchorRemainingAccounts: registerOAppAccounts,
      } satisfies instructions.InitBridgeInstructionAccounts,
      {
        params: {
          portfolio: new PublicKey(
            "CUmdZmnaTZh8g7oFPbQxh3GHPtSVz9Wyw1RXxmUeUxeQ"
          ),
          mainnetRfq: new PublicKey(
            "CUmdZmnaTZh8g7oFPbQxh3GHPtSVz9Wyw1RXxmUeUxeQ"
          ),
          defaultChainId: 12,
          endpointProgram: endpoint.program,
        } satisfies types.InitBridgeParams,
      } satisfies instructions.InitBridgeInstructionArgs,
      this.program
    );
  }

  setRemote(
    admin: PublicKey,
    dstAddress: Uint8Array,
    dstEid: number
  ): TransactionInstruction {
    const [remotePDA] = this.deriver.remote(dstEid);
    return instructions.createSetRemoteInstruction(
      {
        admin,
        bridge: this.idPDA()[0],
        remote: remotePDA,
      } satisfies instructions.SetRemoteInstructionAccounts,
      {
        params: {
          dstEid,
          remote: Array.from(dstAddress),
        } satisfies types.SetRemoteParams,
      },
      this.program
    );
  }

  async getRemote(
    connection: Connection,
    dstEid: number,
    commitmentOrConfig?: Commitment | GetAccountInfoConfig
  ): Promise<Uint8Array | null> {
    const [remotePDA] = this.deriver.remote(dstEid);
    const info = await connection.getAccountInfo(remotePDA, commitmentOrConfig);
    if (info) {
      const remote = await accounts.Remote.fromAccountAddress(
        connection,
        remotePDA,
        commitmentOrConfig
      );
      return Uint8Array.from(remote.address);
    }
    return null;
  }

  async getEndpoint(connection: Connection): Promise<EndpointProgram.Endpoint> {
    if (this.endpoint) {
      return this.endpoint;
    }
    const [id] = this.deriver.bridge();
    const info = await accounts.Bridge.fromAccountAddress(connection, id);
    const programAddr = info.endpointProgram;
    const endpoint = new EndpointProgram.Endpoint(programAddr);
    this.endpoint = endpoint;
    return endpoint;
  }

  async getSendLibraryProgram(
    connection: Connection,
    payer: PublicKey,
    dstEid: number,
    endpoint?: EndpointProgram.Endpoint
  ): Promise<SimpleMessageLibProgram.SimpleMessageLib | UlnProgram.Uln> {
    if (!endpoint) {
      endpoint = await this.getEndpoint(connection);
    }
    const [id] = this.idPDA();
    const sendLibInfo = await endpoint.getSendLibrary(
      connection as any,
      id,
      dstEid
    );
    if (!sendLibInfo?.programId) {
      throw new Error(
        "Send library not initialized or blocked message library"
      );
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

  async send(
    connection: Connection,
    payer: PublicKey,
    // fee: EndpointProgram.types.MessagingFee,
    // mint: PublicKey | null, // Token mint account
    dstEid: number,
    remainingAccounts?: AccountMeta[],
    commitmentOrConfig: Commitment | GetAccountInfoConfig = "confirmed"
  ): Promise<TransactionInstruction> {
    const endpoint = await this.getEndpoint(connection);
    const msgLibProgram = await this.getSendLibraryProgram(
      connection,
      payer,
      dstEid,
      endpoint
    );
    const [bridgePDA] = this.deriver.bridge();
    const [remotePDA] = this.deriver.remote(dstEid);
    const [endpointSettingPDA] = endpoint.deriver.setting();
    const receiverInfo = await accounts.Remote.fromAccountAddress(
      connection,
      remotePDA,
      commitmentOrConfig
    );
    const packetPath: PacketPath = {
      srcEid: 0,
      dstEid,
      sender: hexlify(bridgePDA.toBytes()),
      receiver: hexlify(receiverInfo.address),
    };

    const ix = instructions.createSendInstruction(
      {
        remote: remotePDA,
        bridge: bridgePDA,
        endpoint: endpointSettingPDA,
        // Get remaining accounts from msgLib(simple_msgLib or uln)
        anchorRemainingAccounts:
          remainingAccounts ??
          (await endpoint.getSendIXAccountMetaForCPI(
            connection as any,
            payer,
            packetPath,
            msgLibProgram,
            commitmentOrConfig
          )),
      } satisfies instructions.SendInstructionAccounts,
      {
        params: {
          dstEid: dstEid,
          message: xferMessage,
        } satisfies types.SendParams,
      } satisfies instructions.SendInstructionArgs,
      this.program
    );
    return ix;
  }

  async newQuote(
    connection: Connection,
    payer: PublicKey,

    dstEid: number,
    remainingAccounts?: AccountMeta[],
    commitmentOrConfig: Commitment | GetAccountInfoConfig = "confirmed"
  ): Promise<TransactionInstruction> {
    const endpoint = await this.getEndpoint(connection);
    const msgLibProgram = await this.getSendLibraryProgram(
      connection,
      payer,
      dstEid,
      endpoint
    );
    const [bridgePDA] = this.deriver.bridge();
    const [remotePDA] = this.deriver.remote(dstEid);
    const [endpointSettingPDA] = endpoint.deriver.setting();
    const receiverInfo = await accounts.Remote.fromAccountAddress(
      connection,
      remotePDA,
      commitmentOrConfig
    );
    const packetPath: PacketPath = {
      srcEid: 42168,
      dstEid,
      sender: hexlify(bridgePDA.toBytes()),
      receiver: hexlify(receiverInfo.address),
    };

    const ix = instructions.createQuoteInstruction(
      {
        bridge: bridgePDA,
        endpoint: endpointSettingPDA,
        // Get remaining accounts from msgLib(simple_msgLib or uln)
        anchorRemainingAccounts:
          remainingAccounts ??
          (await endpoint.getSendIXAccountMetaForCPI(
            connection as any,
            payer,
            packetPath,
            msgLibProgram,
            commitmentOrConfig
          )),
      } satisfies instructions.QuoteInstructionAccounts,
      {
        params: {
          dstEid: dstEid,
          receiver: receiverInfo.address,
          message: xferMessage,
          // payInLzToken: false,
          // sender: signer.publicKey,
          // message: new TextEncoder().encode("Hello"),
          // options,
        } satisfies types.QuoteParams,
      } satisfies instructions.QuoteInstructionArgs,
      this.program
    );
    return ix;
  }
}
