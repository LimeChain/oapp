import { EndpointProgram, UlnProgram } from "@layerzerolabs/lz-solana-sdk-v2";
import { Connection, Keypair, PublicKey } from "@solana/web3.js";
import { Bridge } from "../generated/bridge";
import { EndpointId } from "@layerzerolabs/lz-definitions";

const endpointProgram = new EndpointProgram.Endpoint(
  new PublicKey("76y77prsiCMvXMjuoZ5VRrhG5qYBrUMYTE5WgHqgjEn6")
); // endpoint program id, mainnet and testnet are the same
const ulnProgram = new UlnProgram.Uln(
  new PublicKey("7a4WjyR8VZ7yZz5XJAKm39BUGn5iT9CKcv2pmG9tdXVH")
); // uln program id, mainnet and testnet are the same
const executorProgram = new PublicKey(
  "6doghB248px58JSSwG4qejQ46kFMW4AMj7vzJnWZHNZn"
); // executor program id, mainnet and testnet are the same

const bridgeProgram = new Bridge(
  new PublicKey("JAP9nCPz8FSQE5ZQY16yhxq1BMbseJnbMViAAtQWAsSN")
);

const connection = new Connection("https://api.devnet.solana.com");

const signer = Keypair.fromSecretKey(
  new Uint8Array() // solana secret key
);
// evm counter => 0x6BDF6d786b93314f02BB8B098e6E86F83e397322
const remotePeers: { [key in EndpointId]?: string } = {
  // [EndpointId.SEPOLIA_V2_TESTNET]: "0x49b2fa6E068c37516E64E3e7a216C7C266703F90",
  [EndpointId.AMOY_V2_TESTNET]: "0xB8bbaAA516987d610CB9F73F1fd52D634d1c1560",
};

export {
  endpointProgram,
  ulnProgram,
  executorProgram,
  bridgeProgram,
  connection,
  signer,
  remotePeers,
};
