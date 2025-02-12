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
  new PublicKey("CUmdZmnaTZh8g7oFPbQxh3GHPtSVz9Wyw1RXxmUeUxeQ")
);

const connection = new Connection("https://api.devnet.solana.com");

const signer = Keypair.fromSecretKey(
  new Uint8Array([
    239, 59, 47, 115, 199, 2, 168, 51, 43, 205, 152, 119, 158, 25, 243, 254,
    241, 224, 82, 56, 71, 214, 25, 169, 23, 130, 66, 249, 233, 247, 123, 166,
    72, 82, 87, 43, 126, 83, 99, 181, 180, 246, 125, 162, 205, 159, 126, 47, 44,
    149, 15, 136, 204, 64, 79, 252, 107, 198, 74, 129, 153, 196, 96, 12,
  ])
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
