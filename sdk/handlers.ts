import { AnchorProvider, BN, Program, Wallet, web3 } from "@coral-xyz/anchor";
import {
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  TransactionConfirmationStrategy,
} from "@solana/web3.js";
import * as fs from "fs";
import {
  getLayerzeroSendRemainingAccounts,
  getUserInput,
  padSymbol,
  printTransactionEvents,
} from "./utils";

import { getAccountPubKey } from "./utils";
import { Portfolio } from "../target/types/portfolio";
import {
  getAccount,
  getAssociatedTokenAddress,
  getOrCreateAssociatedTokenAccount,
  mintTo,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import { createMint } from "@solana/spl-token";
import { green } from "kleur";
import BridgePDADeriver from "./bridge-pda-deriver";
import { EndpointProgram } from "@layerzerolabs/lz-solana-sdk-v2";
import { ADMIN_SEED, PORTFOLIO_SEED } from "./seed";

const mainnetRFQ = new PublicKey(
  "CUmdZmnaTZh8g7oFPbQxh3GHPtSVz9Wyw1RXxmUeUxeQ"
);
const endpointProgram = new EndpointProgram.Endpoint(
  new PublicKey("76y77prsiCMvXMjuoZ5VRrhG5qYBrUMYTE5WgHqgjEn6")
);
const Spinner = require("cli-spinner").Spinner;
const spinner = new Spinner(green("%s Processing.."));
spinner.setSpinnerString(20);

export const createKeypair = async (keyPath: string): Promise<Keypair> => {
  if (!keyPath) {
    console.clear();
    keyPath = await getUserInput("Enter the path to the wallet file: ");
  }

  if (fs.existsSync(keyPath)) {
    throw new Error("Wallet file already exists");
  }

  let keypair: Keypair = Keypair.generate();

  const secretKeyArray = Array.from(keypair.secretKey);
  fs.writeFileSync(keyPath, JSON.stringify(secretKeyArray));

  return keypair;
};

export const loadKeypair = async (keyPath: string): Promise<Keypair> => {
  if (!keyPath) {
    console.clear();
    keyPath = await getUserInput("Enter the path to the wallet file: ");
  }

  if (!fs.existsSync(keyPath)) {
    throw new Error("Wallet file does not exist");
  }

  const secretKeyArray = JSON.parse(fs.readFileSync(keyPath, "utf-8"));
  const secretKey = new Uint8Array(secretKeyArray);

  return Keypair.fromSecretKey(secretKey);
};

export const printWallet = async (wallet: Wallet) => {
  console.clear();
  console.log(green(`Wallet address: ${wallet.publicKey.toString()}\n\n`));
};

export const airdrop = async (
  wallet: Wallet,
  provider: AnchorProvider,
  connection: Connection
) => {
  const amount = Number(
    await getUserInput("Enter the amount of SOL to airdrop: ")
  );
  try {
    spinner.start();
    const signature = await connection.requestAirdrop(
      wallet.publicKey,
      amount * web3.LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(
      { signature } as TransactionConfirmationStrategy,
      "finalized"
    );
    spinner.stop();
    console.clear();
    console.log(green(`Airdrop request sent with signature: ${signature}\n\n`));
  } catch (err) {
    spinner.stop(true);
    throw err;
  }
};

export const showBalance = async (wallet: Wallet, provider: AnchorProvider) => {
  try {
    spinner.start();
    const balance = await provider.connection.getBalance(wallet.publicKey);
    spinner.stop();
    console.clear();
    console.log(green(`Balance: ${balance / web3.LAMPORTS_PER_SOL} SOL\n\n`));
  } catch (err) {
    spinner.stop(true);
    throw err;
  }
};

export const getParsedTokenList = async (program: Program<Portfolio>) => {
  try {
    spinner.start();
    const tokenListPDA = getAccountPubKey(program, [
      Buffer.from("token_list"),
      Buffer.from("0"),
    ]);
    let tokenListAccount = await program.account.tokenList.fetch(tokenListPDA);

    const tokens = tokenListAccount.tokens.map((tokenBuffer) => {
      return Buffer.from(tokenBuffer).toString().replace(/\0/g, "");
    });
    spinner.stop();
    console.clear();
    console.log(green(`Supported tokens: ${tokens.join(", ")}\n\n`));
  } catch (err) {
    spinner.stop(true);
    throw err;
  }
};

// export const getTokenDetails = async (program: Program<Portfolio>) => {
//   const symbol = (
//     await getUserInput("Enter the symbol of the token: ")
//   ).toUpperCase();
//   const symbolPadded = padSymbol(symbol);
//   try {
//     spinner.start();
//     const [tokenDetails, tokenDetailsBump] =
//       web3.PublicKey.findProgramAddressSync(
//         [Buffer.from("token_details"), symbolPadded],
//         program.programId
//       );
//     let tokenDetailsAccount = await program.account.tokenDetails.fetch(
//       tokenDetails
//     );
//     spinner.stop();
//     console.clear();
//     console.log(
//       green(
//         `Token: ${symbol}\nToken decimals: ${tokenDetailsAccount.decimals}\nToken address: ${tokenDetailsAccount.tokenAddress}\n`
//       )
//     );
//   } catch (err) {
//     spinner.stop(true);
//     throw err;
//   }
// };

// export const addToken = async (
//   program: Program<Portfolio>,
//   connection: Connection,
//   authority: Keypair
// ) => {
//   const tokenSymbol = (
//     await getUserInput("Enter the symbol of the token: ")
//   ).toUpperCase();
//   const tokenDecimals = Number(
//     await getUserInput("Enter the decimals of the token (max 9): ")
//   );

//   if (tokenDecimals > 9) {
//     throw new Error("Decimals must be less or equal to 9");
//   }

//   const symbolPadded = padSymbol(tokenSymbol);
//   try {
//     spinner.start();
//     // Derive PDAs using the helper function
//     const adminPDA = getAccountPubKey(program, [
//       Buffer.from("admin"),
//       authority.publicKey.toBuffer(),
//     ]);

//     const [tokenDetails, tokenDetailsBump] =
//       web3.PublicKey.findProgramAddressSync(
//         [Buffer.from("token_details"), symbolPadded],
//         program.programId
//       );

//     const tokenListPDA = getAccountPubKey(program, [
//       Buffer.from("token_list"),
//       Buffer.from("0"),
//     ]);

//     const splVaultPDA = getAccountPubKey(program, [Buffer.from("spl_vault")]);

//     // Create a new SPL Token Mint for testing
//     const newTokenMint = await createMint(
//       connection,
//       authority, // Payer
//       authority.publicKey, // Mint authority
//       null, // Freeze authority
//       tokenDecimals // Decimals
//     );

//     await connection.getLatestBlockhash({ commitment: "finalized" });

//     // Call the add_token instruction
//     const tx = await program.methods
//       .addToken(Array.from(symbolPadded), newTokenMint, tokenDecimals)
//       .accounts({
//         authority: authority.publicKey,
//         // @ts-ignore
//         admin: adminPDA,
//         portfolio: program.programId,
//         token_details: tokenDetails,
//         tokenMint: newTokenMint,
//         system_program: web3.SystemProgram.programId,
//         token_program: TOKEN_PROGRAM_ID,
//         associated_token_program: TOKEN_PROGRAM_ID,
//         splVault: splVaultPDA,
//       })
//       .remainingAccounts([
//         { pubkey: tokenListPDA, isSigner: false, isWritable: true },
//       ])
//       .signers([authority])
//       .rpc({ commitment: "finalized" });

//     spinner.stop();
//     console.clear();
//     console.log(green(`Token ${tokenSymbol} added successfully\n\n`));
//     await printTransactionEvents(program, tx);
//   } catch (err) {
//     spinner.stop(true);
//     throw err;
//   }
// };

// export const removeToken = async (
//   program: Program<Portfolio>,
//   authority: Keypair
// ) => {
//   const symbol = (
//     await getUserInput("Enter the symbol of the token to remove: ")
//   ).toUpperCase();
//   const symbolPadded = Buffer.alloc(32);
//   symbolPadded.set(Buffer.from(symbol));
//   try {
//     spinner.start();
//     // Derive PDAs
//     const [adminPDA, adminBump] = web3.PublicKey.findProgramAddressSync(
//       [Buffer.from("admin"), authority.publicKey.toBuffer()],
//       program.programId
//     );

//     // Derive the sol_details PDA
//     const [tokenDetails, solDetailsBump] =
//       web3.PublicKey.findProgramAddressSync(
//         [Buffer.from("token_details"), symbolPadded],
//         program.programId
//       );

//     const [tokenList, tokenListBump] = web3.PublicKey.findProgramAddressSync(
//       [Buffer.from("token_list"), Buffer.from("0")],
//       program.programId
//     );

//     const tx = await program.methods
//       //@ts-ignore
//       .removeToken(symbolPadded)
//       .accounts({
//         authority: authority.publicKey, // Pass the authority (payer account)
//         //@ts-ignore
//         tokenDetails,
//         admin: adminPDA,
//         receiver: authority.publicKey,
//         systemProgram: web3.SystemProgram.programId,
//       })
//       .remainingAccounts([
//         {
//           pubkey: tokenList,
//           isWritable: true,
//           isSigner: false,
//         },
//       ])
//       .signers([authority])
//       .rpc({ commitment: "finalized" });

//     spinner.stop();
//     console.clear();
//     console.log(green(`Token ${symbol} removed successfully\n\n`));
//     await printTransactionEvents(program, tx);
//   } catch (err) {
//     spinner.stop(true);
//     throw err;
//   }
// };

export const getProgramBalance = async (
  connection: Connection,
  program: Program<Portfolio>
) => {
  try {
    spinner.start();
    const nativeVaultPDA = getAccountPubKey(program, [
      Buffer.from("sol_vault"),
    ]);
    const balance = await connection.getBalance(nativeVaultPDA);

    spinner.stop();
    console.clear();
    console.log(
      green(`Program balance: ${balance / web3.LAMPORTS_PER_SOL} SOL\n\n`)
    );
  } catch (err) {
    spinner.stop(true);
    throw err;
  }
};

// export const depositSOL = async (
//   program: Program<Portfolio>,
//   authority: Keypair
// ) => {
//   const amount = Number(
//     await getUserInput("Enter the amount of SOL to deposit: ")
//   );
//   try {
//     spinner.start();
//     const nativeVaultPDA = getAccountPubKey(program, [
//       Buffer.from("sol_vault"),
//     ]);

//     const lamports = web3.LAMPORTS_PER_SOL * amount;

//     const tx = await program.methods
//       .depositNative(new BN(lamports))
//       .accounts({
//         // @ts-ignore
//         user: authority.publicKey,
//         // @ts-ignore
//         nativeVault: nativeVaultPDA,
//         tokenProgram: TOKEN_PROGRAM_ID,
//         systemProgram: web3.SystemProgram.programId,
//       })
//       .signers([authority])
//       .rpc({ commitment: "finalized" });

//     spinner.stop();
//     console.clear();
//     console.log(green(`Deposited ${amount} SOL to ${nativeVaultPDA}\n\n`));
//     await printTransactionEvents(program, tx);
//   } catch (err) {
//     spinner.stop(true);
//     throw err;
//   }
// };

// export const depositSPLToken = async (
//   program: Program<Portfolio>,
//   authority: Keypair,
//   connection: Connection
// ) => {
//   const tokenSymbol = (
//     await getUserInput("Enter the symbol of the token: ")
//   ).toUpperCase();
//   const symbolPadded = padSymbol(tokenSymbol);
//   const amount = Number(
//     await getUserInput("Enter the amount of tokens to deposit: ")
//   );

//   try {
//     spinner.start();
//     const tokenDetailsPDA = getAccountPubKey(program, [
//       Buffer.from("token_details"),
//       symbolPadded,
//     ]);

//     const splVaultPDA = getAccountPubKey(program, [Buffer.from("spl_vault")]);

//     const tokenDetails = await program.account.tokenDetails.fetch(
//       tokenDetailsPDA
//     );

//     if (!tokenDetails.tokenAddress) {
//       throw new Error("Token address not found!");
//     }

//     const userATA = await getOrCreateAssociatedTokenAccount(
//       connection,
//       authority,
//       tokenDetails.tokenAddress,
//       authority.publicKey, // User authority
//       true, // User authority
//       "finalized",
//       { commitment: "finalized" }
//     );

//     // Fetch the program's vault associated token account
//     const vaultATA = await getOrCreateAssociatedTokenAccount(
//       connection,
//       authority,
//       tokenDetails.tokenAddress,
//       splVaultPDA,
//       true,
//       "finalized",
//       { commitment: "finalized" }
//     );

//     // console.log(new PublicKey("DD12vMyLdwszDCAzLhsUPwBmzJXv611dUCPhqwpZQYG4"));
//     // console.log(BridgePDADeriver.bridge_remote(40267));
//     // console.log(BridgePDADeriver.layerZeroEndpoint(40267));
//     const bridgePDA = new PublicKey(
//       "BmmRy2hjqxeMERH3R4arTdohBCC7cjhqPNiAq52iGpNj"
//     );
//     const remotePDA = BridgePDADeriver.bridge_remote(40267)[0];

//     const remainingAccounts = await getLayerzeroSendRemainingAccounts(
//       connection,
//       40168,
//       40267,
//       authority.publicKey,
//       bridgePDA,
//       remotePDA
//     );

//     console.log(remainingAccounts.length);

//     const tx = await program.methods
//       .deposit(
//         Array.from(symbolPadded),
//         new BN(amount * 10 ** tokenDetails.decimals)
//       )
//       .accounts({
//         user: authority.publicKey,
//         // @ts-ignore
//         tokenDetails: tokenDetailsPDA,
//         from: userATA.address,
//         to: vaultATA.address,
//         tokenProgram: TOKEN_PROGRAM_ID,
//         splVault: splVaultPDA,
//         bridge: bridgePDA,
//         remote: remotePDA,
//         endpointProgram: BridgePDADeriver.layerZeroEndpoint(40267)[0],
//         bridgeProgram: new PublicKey(
//           "DD12vMyLdwszDCAzLhsUPwBmzJXv611dUCPhqwpZQYG4"
//         ),
//       })
//       .remainingAccounts(remainingAccounts)
//       .signers([authority])
//       .rpc({ commitment: "finalized" });

//     spinner.stop();
//     console.clear();
//     console.log(
//       green(`Deposited ${amount} ${tokenSymbol} to ${vaultATA.address}\n\n`)
//     );
//     await printTransactionEvents(program, tx);
//   } catch (err) {
//     spinner.stop(true);
//     throw err;
//   }
// };

// export const mintSPLToken = async (
//   program: Program<Portfolio>,
//   authority: Keypair,
//   connection: Connection
// ) => {
//   const tokenSymbol = (
//     await getUserInput("Enter the symbol of the token: ")
//   ).toUpperCase();
//   const symbolPadded = padSymbol(tokenSymbol);
//   const amount = Number(
//     await getUserInput("Enter the amount of tokens to mint: ")
//   );

//   try {
//     spinner.start();
//     const tokenDetailsPDA = getAccountPubKey(program, [
//       Buffer.from("token_details"),
//       symbolPadded,
//     ]);

//     const tokenDetails = await program.account.tokenDetails.fetch(
//       tokenDetailsPDA
//     );
//     if (!tokenDetails.tokenAddress) {
//       throw new Error("Token address not found!");
//     }

//     const userATA = await getOrCreateAssociatedTokenAccount(
//       connection,
//       authority,
//       tokenDetails.tokenAddress,
//       authority.publicKey,
//       true, // User authority,
//       "finalized",
//       { commitment: "finalized" }
//     );

//     await mintTo(
//       connection,
//       authority,
//       tokenDetails.tokenAddress,
//       userATA.address,
//       authority.publicKey, // Mint authority
//       amount * 10 ** tokenDetails.decimals // Amount in base units
//     );

//     spinner.stop();
//     console.clear();
//     console.log(green(`Minted ${amount} tokens to ${userATA.address}\n\n`));
//   } catch (err) {
//     spinner.stop(true);
//     throw err;
//   }
// };

// export const checkSPLTokenBalance = async (
//   program: Program<Portfolio>,
//   authority: Keypair,
//   connection: Connection
// ) => {
//   const tokenSymbol = (
//     await getUserInput("Enter the symbol of the token: ")
//   ).toUpperCase();
//   const symbolPadded = padSymbol(tokenSymbol);

//   try {
//     spinner.start();
//     const tokenDetailsPDA = getAccountPubKey(program, [
//       Buffer.from("token_details"),
//       symbolPadded,
//     ]);

//     const tokenDetails = await program.account.tokenDetails.fetch(
//       tokenDetailsPDA
//     );

//     if (!tokenDetails.tokenAddress) {
//       throw new Error("Token address not found!");
//     }

//     const tokenAccountAddress = await getAssociatedTokenAddress(
//       tokenDetails.tokenAddress,
//       authority.publicKey
//     );

//     // Fetch token account info
//     const tokenAccount = await getAccount(connection, tokenAccountAddress);

//     spinner.stop();
//     console.clear();
//     console.log(
//       green(
//         `Balance: ${
//           Number(tokenAccount.amount) / 10 ** tokenDetails.decimals
//         } ${tokenSymbol}\n\n`
//       )
//     );
//   } catch (err) {
//     spinner.stop(true);
//     throw err;
//   }
// };

// export const getSPLTokenVaultBalance = async (
//   program: Program<Portfolio>,
//   authority: Keypair,
//   connection: Connection
// ) => {
//   const tokenSymbol = (
//     await getUserInput("Enter the symbol of the token: ")
//   ).toUpperCase();
//   const symbolPadded = padSymbol(tokenSymbol);

//   try {
//     spinner.start();
//     const tokenDetailsPDA = getAccountPubKey(program, [
//       Buffer.from("token_details"),
//       symbolPadded,
//     ]);

//     const splVaultPDA = getAccountPubKey(program, [Buffer.from("spl_vault")]);

//     const tokenDetails = await program.account.tokenDetails.fetch(
//       tokenDetailsPDA
//     );

//     if (!tokenDetails.tokenAddress) {
//       throw new Error("Token address not found!");
//     }

//     const vaultATA = await getOrCreateAssociatedTokenAccount(
//       connection,
//       authority,
//       tokenDetails.tokenAddress,
//       splVaultPDA,
//       true,
//       "finalized",
//       { commitment: "finalized" }
//     );

//     const decimals = 10 ** tokenDetails.decimals;
//     spinner.stop();
//     console.clear();
//     console.log(
//       green(`Balance: ${Number(vaultATA.amount) / decimals} ${tokenSymbol}\n\n`)
//     );
//   } catch (err) {
//     spinner.stop(true);
//     throw err;
//   }
// };

// export const addAdmin = async (
//   program: Program<Portfolio>,
//   authority: Keypair
// ) => {
//   const newAdminPath = await getUserInput(
//     "Enter the path to the new admin wallet file: "
//   );
//   const newAdmin = await loadKeypair(newAdminPath);
//   const newAdminPDA = getAccountPubKey(program, [
//     Buffer.from("admin"),
//     newAdmin.publicKey.toBuffer(),
//   ]);

//   const adminPDA = getAccountPubKey(program, [
//     Buffer.from("admin"),
//     authority.publicKey.toBuffer(),
//   ]);

//   // Call the addAdmin instruction
//   try {
//     spinner.start();
//     await program.methods
//       .addAdmin(newAdmin.publicKey)
//       .accounts({
//         // @ts-ignore
//         admin: adminPDA,
//         newAdmin: newAdminPDA,
//         systemProgram: web3.SystemProgram.programId,
//         authority: authority.publicKey,
//       })
//       .signers([authority])
//       .rpc({ commitment: "finalized" });

//     spinner.stop();
//     console.clear();
//     console.log(`New admin added: ${newAdmin.publicKey.toBase58()}`);
//   } catch (error) {
//     spinner.stop(true);
//     throw error;
//   }
// };

// export const removeAdmin = async (
//   program: Program<Portfolio>,
//   authority: Keypair
// ) => {
//   const newAdminPath = await getUserInput(
//     "Enter the path to the admin wallet file: "
//   );
//   const newAdmin = await loadKeypair(newAdminPath);

//   const newAdminPDA = getAccountPubKey(program, [
//     Buffer.from("admin"),
//     newAdmin.publicKey.toBuffer(),
//   ]);

//   const adminPDA = getAccountPubKey(program, [
//     Buffer.from("admin"),
//     authority.publicKey.toBuffer(),
//   ]);

//   try {
//     spinner.start();
//     await program.methods
//       .removeAdmin(newAdmin.publicKey)
//       .accounts({
//         // @ts-ignore
//         admin: adminPDA,
//         adminToRemove: newAdminPDA,
//         systemProgram: web3.SystemProgram.programId,
//         receiver: authority.publicKey, // Refund lamports to the remover
//         authority: authority.publicKey,
//       })
//       .signers([authority])
//       .rpc({ commitment: "finalized" });

//     spinner.stop();
//     console.clear();
//     console.log(`Admin removed: ${newAdmin.publicKey.toBase58()}`);
//   } catch (error) {
//     spinner.stop(true);
//     throw error;
//   }
// };

// export const pauseProgram = async (
//   program: Program<Portfolio>,
//   authority: Keypair
// ) => {
//   try {
//     spinner.start();
//     const adminPDA = getAccountPubKey(program, [
//       Buffer.from("admin"),
//       authority.publicKey.toBuffer(),
//     ]);
//     const globalConfigPDA = getAccountPubKey(program, [
//       Buffer.from("global_config"),
//     ]);

//     const tx = await program.methods
//       .setPaused(true)
//       .accounts({
//         authority: authority.publicKey,
//         // @ts-ignore
//         admin: adminPDA,
//         systemProgram: web3.SystemProgram.programId,
//         globalConfig: globalConfigPDA,
//       })
//       .signers([authority])
//       .rpc({ commitment: "finalized" });

//     spinner.stop();
//     console.clear();
//     console.log(green(`Program paused\n\n`));
//   } catch (err) {
//     spinner.stop(true);
//     throw err;
//   }
// };

// export const unpauseProgram = async (
//   program: Program<Portfolio>,
//   authority: Keypair
// ) => {
//   try {
//     spinner.start();
//     const adminPDA = getAccountPubKey(program, [
//       Buffer.from("admin"),
//       authority.publicKey.toBuffer(),
//     ]);
//     const globalConfigPDA = getAccountPubKey(program, [
//       Buffer.from("global_config"),
//     ]);

//     await program.methods
//       .setPaused(false)
//       .accounts({
//         authority: authority.publicKey,
//         // @ts-ignore
//         admin: adminPDA,
//         systemProgram: web3.SystemProgram.programId,
//         globalConfig: globalConfigPDA,
//       })
//       .signers([authority])
//       .rpc({ commitment: "finalized" });

//     spinner.stop();
//     console.clear();
//     console.log(green(`Program unpaused\n\n`));
//   } catch (err) {
//     spinner.stop(true);
//     throw err;
//   }
// };

// export const banAccount = async (
//   program: Program<Portfolio>,
//   authority: Keypair
// ) => {
//   const input = await getUserInput(
//     "Enter the Public key of the account to ban: "
//   );
//   try {
//     spinner.start();
//     const banReason = { abuse: {} };
//     const bannedAccount = new web3.PublicKey(input);

//     const bannedAccountPDA = getAccountPubKey(program, [
//       Buffer.from("banned"),
//       bannedAccount.toBuffer(),
//     ]);

//     const adminPDA = getAccountPubKey(program, [
//       Buffer.from("admin"),
//       authority.publicKey.toBuffer(),
//     ]);

//     const tx = await program.methods
//       .banAccount(bannedAccount, banReason)
//       .accounts({
//         //@ts-ignore
//         admin: adminPDA,
//         bannedAccount: bannedAccountPDA,
//         systemProgram: web3.SystemProgram.programId,
//         authority: authority.publicKey,
//       })
//       .signers([authority])
//       .rpc({ commitment: "finalized" });

//     spinner.stop();
//     console.clear();
//     console.log(green(`Account ${bannedAccount.toBase58()} banned\n\n`));
//     await printTransactionEvents(program, tx);
//   } catch (err) {
//     spinner.stop(true);
//     throw err;
//   }
// };

// export const unbanAccount = async (
//   program: Program<Portfolio>,
//   authority: Keypair
// ) => {
//   const input = await getUserInput(
//     "Enter the Public key of the account to unban: "
//   );
//   try {
//     spinner.start();
//     const bannedAccount = new web3.PublicKey(input);

//     const bannedAccountPDA = getAccountPubKey(program, [
//       Buffer.from("banned"),
//       bannedAccount.toBuffer(),
//     ]);

//     const adminPDA = getAccountPubKey(program, [
//       Buffer.from("admin"),
//       authority.publicKey.toBuffer(),
//     ]);

//     const tx = await program.methods
//       .unbanAccount(bannedAccount)
//       .accounts({
//         //@ts-ignore
//         admin: adminPDA,
//         bannedAccount: bannedAccountPDA,
//         systemProgram: web3.SystemProgram.programId,
//         authority: authority.publicKey,
//         receiver: authority.publicKey,
//       })
//       .signers([authority])
//       .rpc({ commitment: "finalized" });

//     spinner.stop();
//     console.clear();
//     console.log(green(`Account ${bannedAccount.toBase58()} unbanned\n\n`));
//     await printTransactionEvents(program, tx);
//   } catch (err) {
//     spinner.stop(true);
//     throw err;
//   }
// };

// export const withdrawNative = async (
//   program: Program<Portfolio>,
//   user: Keypair
// ) => {
//   try {
//     const symbol = Buffer.from("SOL");
//     const symbolPadded = Buffer.alloc(32);
//     symbolPadded.set(symbol);

//     const input = await getUserInput("Enter the amount of SOL to withdraw: ");
//     spinner.start();
//     const lamports = Number(input) * LAMPORTS_PER_SOL;
//     const xfer = {
//       nonce: new BN(0),
//       transaction: { withdraw: {} },
//       trader: user.publicKey,
//       symbol: Array.from(symbolPadded),
//       quantity: new BN(lamports),
//       timestamp: new BN(Date.now()),
//       // 28 bytes array
//       customdata: Array.from(new Uint8Array(28)),
//     };
//     const nativeVaultPDA = getAccountPubKey(program, [
//       Buffer.from("sol_vault"),
//     ]);
//     const globalConfigPDA = getAccountPubKey(program, [
//       Buffer.from("global_config"),
//     ]);

//     // Withdraw 1 SOL from the native vault
//     const tx = await program.methods
//       .withdrawNative(xfer)
//       .accounts({
//         to: user.publicKey,
//         // @ts-ignore
//         nativeVault: nativeVaultPDA,
//         systemProgram: web3.SystemProgram.programId,
//         globalConfig: globalConfigPDA,
//         instructionSysvarAccount: web3.SYSVAR_INSTRUCTIONS_PUBKEY,
//       })
//       .signers([user])
//       .rpc({ commitment: "finalized" });

//     spinner.stop();
//     console.clear();
//     console.log(green(`Withdrew ${input} SOL\n\n`));

//     await printTransactionEvents(program, tx);
//   } catch (err) {
//     spinner.stop(true);
//     throw err;
//   }
// };

// export const withdrawToken = async (
//   program: Program<Portfolio>,
//   user: Keypair,
//   connection: Connection
// ) => {
//   const tokenSymbol = (
//     await getUserInput("Enter the symbol of the token: ")
//   ).toUpperCase();
//   const symbolPadded = padSymbol(tokenSymbol);
//   const amount = Number(
//     await getUserInput("Enter the amount of tokens to withdraw: ")
//   );

//   try {
//     spinner.start();
//     const tokenDetailsPDA = getAccountPubKey(program, [
//       Buffer.from("token_details"),
//       symbolPadded,
//     ]);

//     const splVaultPDA = getAccountPubKey(program, [Buffer.from("spl_vault")]);

//     const tokenDetails = await program.account.tokenDetails.fetch(
//       tokenDetailsPDA
//     );

//     if (!tokenDetails.tokenAddress) {
//       throw new Error("Token address not found!");
//     }

//     const userATA = await getOrCreateAssociatedTokenAccount(
//       connection,
//       user,
//       tokenDetails.tokenAddress,
//       user.publicKey, // User authority
//       true, // User authority
//       "finalized",
//       { commitment: "finalized" }
//     );

//     // Fetch the program's vault associated token account
//     const vaultATA = await getOrCreateAssociatedTokenAccount(
//       connection,
//       user,
//       tokenDetails.tokenAddress,
//       splVaultPDA,
//       true,
//       "finalized",
//       { commitment: "finalized" }
//     );

//     const xfer = {
//       nonce: new BN(0),
//       transaction: 1,
//       trader: Array(20).fill(0),
//       symbol: Array.from(symbolPadded),
//       quantity: [new BN(amount * 10 ** tokenDetails.decimals), 0, 0, 0],
//       timestamp: new BN(Date.now()),
//       // 28 bytes array
//       customdata: Array.from(new Uint8Array(28)),
//       messageType: 0,
//     };

//     const tx = await program.methods
//       .withdraw(xfer)
//       .accounts({
//         from: vaultATA.address,
//         to: userATA.address,
//         // @ts-ignore
//         tokenProgram: TOKEN_PROGRAM_ID,
//         splVault: splVaultPDA,
//         tokenDetails: tokenDetailsPDA,
//         systemProgram: web3.SystemProgram.programId,
//       })
//       .signers([user])
//       .rpc({ commitment: "finalized" });

//     spinner.stop();
//     console.clear();
//     console.log(
//       green(`Withdrew ${amount} ${tokenSymbol} to ${userATA.address}\n\n`)
//     );

//     await printTransactionEvents(program, tx);
//   } catch (err) {
//     spinner.stop(true);
//     throw err;
//   }
// };

// export const setBridgeAddress = async (
//   program: Program<Portfolio>,
//   admin: Keypair
// ) => {
//   const input = await getUserInput("Enter the bridge address\n");
//   spinner.start();
//   try {
//     const bridgePubkey = new PublicKey(input);
//     await program.methods
//       .setPortfolioBridge(bridgePubkey)
//       .signers([admin])
//       .rpc({ commitment: "finalized" });
//     spinner.stop();
//     console.clear();
//     console.log(green(`Bridge address updated to: ${input}`));
//   } catch (err) {
//     spinner.stop(true);
//     throw err;
//   }
// };

export const init = async (program: Program<Portfolio>, admin: Keypair) => {
  spinner.start();
  try {
    const adminPDA = getAccountPubKey(program, [
      Buffer.from(ADMIN_SEED),
      admin.publicKey.toBuffer(),
    ]);

    const splVaultPDA = getAccountPubKey(program, [Buffer.from("spl_vault")]);
    const tokenListPDA = getAccountPubKey(program, [
      Buffer.from("token_list"),
      Buffer.from("0"),
    ]);
    const portfolioPDA = getAccountPubKey(program, [
      Buffer.from(PORTFOLIO_SEED),
    ]);

    const register_reamining_accounts =
      endpointProgram.getRegisterOappIxAccountMetaForCPI(
        admin.publicKey,
        portfolioPDA
      );
    await program.methods
      .initialize({
        srcChainId: 40168,
        mainnetRfq: mainnetRFQ,
        defaultChainId: 42,
        endpointProgram: endpointProgram.program,
      })
      .accounts({
        //@ts-ignore
        portfolio: portfolioPDA,
        splVault: splVaultPDA,
        admin: adminPDA,
        authority: admin.publicKey,
        systemProgram: web3.SystemProgram.programId,
        endpointProgram: endpointProgram.program,
      })
      .remainingAccounts(register_reamining_accounts)
      .signers([admin])
      .rpc({ commitment: "finalized" });

    spinner.stop();
    console.clear();
    console.log("Portfolio initialized");
  } catch (err) {
    spinner.stop(true);
    throw err;
  }
};
