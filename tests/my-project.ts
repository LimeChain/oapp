import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MyProject } from "../target/types/my_project";

describe("my-project", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.MyProject as Program<MyProject>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
    // Configure the client to use the local cluster
    const provider = anchor.AnchorProvider.local();
    anchor.setProvider(provider);
    const authority = provider.wallet.publicKey;
    const payer = anchor.web3.Keypair.generate();
    const fakeAdmin = anchor.web3.Keypair.generate();
    let mint: anchor.web3.PublicKey;
  });
});
