
import  {web3, Wallet, AnchorProvider, setProvider, Program, Address} from '@coral-xyz/anchor';
import { EndpointProgram, EventPDADeriver } from '@layerzerolabs/lz-solana-sdk-v2';

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
      [Buffer.from("global_config")],
      program.programId
    );
    console.log("Global config pubkey:", gc.toBase58());

    const [solVault, solVaultBump] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("sol_vault")],
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
