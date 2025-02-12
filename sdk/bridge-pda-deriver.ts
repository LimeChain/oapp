import { PublicKey } from "@solana/web3.js";
import BN from "bn.js";

import {
  EndpointProgram,
  ENDPOINT_SEED,
  oappIDPDA,
} from "@layerzerolabs/lz-solana-sdk-v2";

const BRIDGE_SEED = "Bridge";
const REMOTE_SEED = "Remote";
const SOL_VAULT_SEED = "SolVault";

const BRIDGE_PROGRAM_ID = new PublicKey(
  "DD12vMyLdwszDCAzLhsUPwBmzJXv611dUCPhqwpZQYG4"
);

const endpointProgram = new EndpointProgram.Endpoint(
  new PublicKey("76y77prsiCMvXMjuoZ5VRrhG5qYBrUMYTE5WgHqgjEn6")
);

class BridgePDADeriver {
  constructor(public readonly program: PublicKey) {}

  bridge(): [PublicKey, number] {
    return oappIDPDA(this.program, BRIDGE_SEED);
  }

  layerZeroEndpoint(dstChainId: number): [PublicKey, number] {
    return oappIDPDA(
      new PublicKey("76y77prsiCMvXMjuoZ5VRrhG5qYBrUMYTE5WgHqgjEn6"), // ENDPOINT_ID
      ENDPOINT_SEED
    );
  }

  bridge_remote(dstChainId: number): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
      [
        Buffer.from(REMOTE_SEED),
        new BN(dstChainId).toArrayLike(Buffer, "be", 4),
      ],
      this.program
    );
  }
}

export default new BridgePDADeriver(BRIDGE_PROGRAM_ID);
