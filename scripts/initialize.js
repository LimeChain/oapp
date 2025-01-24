import pkg from '@coral-xyz/anchor';
import lzv2 from '@layerzerolabs/lz-solana-sdk-v2';
const { web3, Wallet, AnchorProvider, setProvider, Program, Address } = pkg;
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
    const idl = JSON.parse(readFileSync("./target/idl/my_project.json", "utf8"));

    // IDL Address
    const programPath = "portfolio-oapp.json"; // Update this path
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

    const [solVault, solVaultBump] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("sol_vault")],
      program.programId
    );

    const endpointPubkey = new web3.PublicKey("76y77prsiCMvXMjuoZ5VRrhG5qYBrUMYTE5WgHqgjEn6")
    const [oappPubkey, oappBump] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("oapp"), program.programId.toBuffer()],
      endpointPubkey
    );

    console.log("Oapp pubkey:", oappPubkey.toBase58());

    console.log("Initializing program...");
    await program.methods
      .initialize(params)
      .accounts({
        authority, // Pass the authority (payer account)
        globalConfig: gc,
        solVault,
        systemProgram: web3.SystemProgram.programId,
      })
      .remainingAccounts([
        {pubkey: endpointPubkey, isWritable: false, isSigner: false},
        {pubkey: oappPubkey, isWritable: true, isSigner: false},
        {pubkey: program.programId, isWritable: true, isSigner: false}, // dunno
        {pubkey: program.programId, isWritable: true, isSigner: false}, // dunno
        {pubkey: authority, isWritable: true, isSigner: true},
        {pubkey: web3.SystemProgram.programId, isWritable: true, isSigner: false}, // dunno
        {pubkey: web3.SystemProgram.programId, isWritable: true, isSigner: false}, // dunno
      ])
      .signers([keypair])
      .rpc();

    console.log("Program initialized successfully!");

  } catch (err) {
    console.error("Error initializing program:", err);
  }
})();
