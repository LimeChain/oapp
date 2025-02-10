
import { web3, Wallet, AnchorProvider, setProvider, Program, Address } from '@coral-xyz/anchor';
import { buildVersionedTransaction, EndpointProgram, EventPDADeriver } from '@layerzerolabs/lz-solana-sdk-v2';
import { Connection, Keypair, Signer, TransactionInstruction } from "@solana/web3.js";
import { EndpointId } from "@layerzerolabs/lz-definitions";

import { readFileSync } from "fs";

(async () => {
  try {
    // Set up a custom provider for Devnet
    const keypairPath = "upgrade-authority.json"; // Update this path
    const keypair = web3.Keypair.fromSecretKey(
        Uint8Array.from(JSON.parse(readFileSync(keypairPath, "utf-8")))
    );

    // Set up a provider with the custom wallet
    const connection = new web3.Connection("https://api.devnet.solana.com", "confirmed");
    const wallet = new Wallet(keypair); // Use the custom wallet
    const provider = new AnchorProvider(connection, wallet, {
      preflightCommitment: "processed",
    });
    setProvider(provider);
    // Load the IDL
    const idl = JSON.parse(readFileSync("./target/idl/oapp-example.json", "utf8"));

    // IDL Address
    const programPath = "portfolio-oapp-example.json"; // Update this path
    const programKeypair = web3.Keypair.fromSecretKey(
        Uint8Array.from(JSON.parse(readFileSync(programPath, "utf-8")))
    );

    // Load the program
    const program = new Program(idl, programKeypair.publicKey, provider);

    // Authority (your wallet public key)
    const authority = keypair.publicKey;

    // Call the `initialize` instruction
    const portfolioPubkey = new web3.PublicKey("9Fmenbf7Qti4sG3hQWwifpAvGArtqtK9N96jdN19MX3u");
    const mainnetrfqPubkey = new web3.PublicKey("9Fmenbf7Qti4sG3hQWwifpAvGArtqtK9N96jdN19MX3u");

    const params = {
      portfolio: portfolioPubkey,
      mainnetrfq: mainnetrfqPubkey,
    }

    const [gc, gcBump] = web3.PublicKey.findProgramAddressSync(
        [Buffer.from("globalConfig")],
        program.programId
    );
    console.log("Global config pubkey:", gc.toBase58());

    const [solVault, solVaultBump] = web3.PublicKey.findProgramAddressSync(
        [Buffer.from("solVault")],
        program.programId
    );
    console.log("Sol vault pubkey:", solVault.toBase58());

    const endpointPubkey = new web3.PublicKey("76y77prsiCMvXMjuoZ5VRrhG5qYBrUMYTE5WgHqgjEn6")
    const endpoint = new EndpointProgram.Endpoint(endpointPubkey)
    const [oAppRegistry] = endpoint.deriver.oappRegistry(solVault);
    const [eventAuthority] = new EventPDADeriver(endpoint.program).eventAuthority()
    const ixAccounts = EndpointProgram.instructions.createRegisterOappInstructionAccounts(
        {
          payer: authority,
          oapp: solVault,
          oappRegistry: oAppRegistry,
          eventAuthority,
          program: endpoint.program,
        },
        endpoint.program,
    );

    const registerOAppAccounts = [
      {
        pubkey: endpoint.program,
        isSigner: false,
        isWritable: false,
      },
      ...ixAccounts,
    ]

    registerOAppAccounts[1].isSigner = false
    registerOAppAccounts[2].isSigner = false

    console.log("Initializing program...");
    await program.methods
        .initialize(params)
        .accounts({
          authority, // Pass the authority (payer account)
          globalConfig: gc,
          solVault,
          systemProgram: web3.SystemProgram.programId,
        })
        .remainingAccounts(registerOAppAccounts)
        .signers([keypair])
        .rpc();

    console.log("Program initialized successfully!");

  } catch (err) {
    console.error("Error initializing program:", err);
  }
})();

async function setPeers(
    connection: Connection,
    admin: Keypair,
    remote: EndpointId,
    remotePeer: Uint8Array
): Promise<void> {
  const ix = endpoint.program.setRemote(admin.publicKey, remotePeer, remote);
  const [remotePDA] = counterProgram.omniCounterDeriver.remote(remote);
  let current = "";
  try {
    const info = await accounts.Remote.fromAccountAddress(
        connection,
        remotePDA,
        {
          commitment: "confirmed",
        }
    );
    current = Buffer.from(info.address).toString("hex");
  } catch (e) {
    /*remote not init*/
  }
  if (current == Buffer.from(remotePeer).toString("hex")) {
    return Promise.resolve();
  }
  await sendAndConfirm(connection, [admin], [ix]);
}

async function sendAndConfirm(
    connection: Connection,
    signers: Signer[],
    instructions: TransactionInstruction[]
): Promise<void> {
  const tx = await buildVersionedTransaction(
      connection as any,
      signers[0].publicKey,
      instructions,
      "confirmed"
  );
  tx.sign(signers);
  const hash = await connection.sendRawTransaction(tx.serialize(), {
    skipPreflight: true,
  });
  console.log(`Tx hash: ${hash}`);
  await connection.confirmTransaction(hash, "confirmed");
}
