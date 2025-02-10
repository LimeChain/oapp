import { PublicKey } from "@solana/web3.js";
import BN from "bn.js";

import { oappIDPDA } from "@layerzerolabs/lz-solana-sdk-v2";

const BRIDGE_SEED = "Bridge";
const REMOTE_SEED = "Remote";
const SOL_VAULT_SEED = "SolVault";

export class PDADeriver {
  constructor(public readonly program: PublicKey) {}

  bridge(): [PublicKey, number] {
    return oappIDPDA(this.program, BRIDGE_SEED);
  }

  solVault(): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
      [Buffer.from(SOL_VAULT_SEED)],
      this.program
    );
  }

  remote(dstChainId: number): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
      [
        Buffer.from(REMOTE_SEED),
        this.bridge()[0].toBytes(),
        new BN(dstChainId).toArrayLike(Buffer, "be", 4),
      ],
      this.program
    );
  }
}
