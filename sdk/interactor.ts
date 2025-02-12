import {
  AnchorProvider,
  Program,
  setProvider,
  Wallet,
} from "@coral-xyz/anchor";
import {
  airdrop,
  showBalance,
  getParsedTokenList,
  // getTokenDetails,
  // addToken,
  // removeToken,
  // depositSOL,
  getProgramBalance,
  createKeypair,
  loadKeypair,
  // depositSPLToken,
  // mintSPLToken,
  printWallet,
  // checkSPLTokenBalance,
  // getSPLTokenVaultBalance,
  // addAdmin,
  // removeAdmin,
  // pauseProgram,
  // unpauseProgram,
  // banAccount,
  // unbanAccount,
  // withdrawNative,
  // withdrawToken,
  init,
  // setBridgeAddress,
} from "./handlers";
import { clusterApiUrl, Connection, Keypair, PublicKey } from "@solana/web3.js";
import IDL from "../target/idl/portfolio.json";
import { red } from "kleur";
import * as fs from "fs";

const DEFAULT_WALLET_PATH = "./vlady.json";

class Interactor {
  private wallet: Wallet | null = null;
  private provider: AnchorProvider | null = null;
  private connection: Connection | null = null;
  private program: Program<any> | null = null;
  private keypair: Keypair | null = null;

  constructor() {
    this.init();
  }

  async init() {
    try {
      await this.createDefaultWallet();

      await this.setupDevnetProvider();
      if (this.provider) {
        setProvider(this.provider);
      }
      await this.setProgram();
    } catch (error) {
      console.error(red(`Error initializing: ${error}\n\n`));
    }
  }

  async setProgram() {
    if (!this.provider) {
      console.error(red("Provider not found"));
      return;
    }
    this.program = new Program(IDL as any, this.provider);
  }

  async loadWallet() {
    try {
      this.keypair = await loadKeypair("");
      this.wallet = new Wallet(this.keypair);
    } catch (error) {
      console.error(red(`Error setting wallet: ${error}\n\n`));
    }
  }

  async createWallet() {
    try {
      this.keypair = await createKeypair("");
      this.wallet = new Wallet(this.keypair);
    } catch (error) {
      console.error(red(`Error setting wallet: ${error}\n\n`));
    }
  }

  async createDefaultWallet() {
    try {
      if (fs.existsSync(DEFAULT_WALLET_PATH)) {
        this.keypair = await loadKeypair(DEFAULT_WALLET_PATH);
      } else {
        this.keypair = await createKeypair(DEFAULT_WALLET_PATH);
      }

      this.wallet = new Wallet(this.keypair);
    } catch (error) {
      console.error(red(`Error setting wallet: ${error}\n\n`));
    }
  }

  setupDevnetProvider = async () => {
    const connection = new Connection(clusterApiUrl("devnet"));
    if (!this.wallet) {
      console.error(red("Wallet not found"));
      return;
    }
    const provider = new AnchorProvider(
      connection,
      this.wallet,
      { commitment: "confirmed" } // You can customize options
    );
    this.provider = provider;
    this.connection = connection;
  };

  printWallet = async () => {
    if (!this.wallet) {
      console.error(red("Wallet not found"));
      return;
    }
    await printWallet(this.wallet);
  };

  requestAirdrop = async () => {
    if (!this.wallet || !this.provider || !this.connection) {
      console.error(red("Wallet, provider, or connection not found"));
      return;
    }
    try {
      await airdrop(this.wallet, this.provider, this.connection);
    } catch (error) {
      console.error(red("Error requesting airdrop\n\n"));
    }
  };

  showBalance = async () => {
    if (!this.wallet || !this.provider || !this.connection) {
      console.error(red("Wallet, provider, or connection not found\n\n"));
      return;
    }
    try {
      await showBalance(this.wallet, this.provider);
    } catch (error) {
      console.error(red("Error showing balance\n\n"));
    }
  };

  getTokenList = async () => {
    if (!this.program || !this.connection) {
      console.error(red("Program not found\n\n"));
      return;
    }
    try {
      await getParsedTokenList(this.program);
    } catch (error) {
      console.error(red("Error getting token list\n\n"));
    }
  };

  getTokenDetails = async () => {
    if (!this.program || !this.connection) {
      console.error(red("Program not found\n\n"));
      return;
    }
    try {
      //await getTokenDetails(this.program);
    } catch (error) {
      console.error(red("Error getting token details\n\n"));
    }
  };

  addToken = async () => {
    if (!this.program || !this.connection || !this.wallet || !this.keypair) {
      console.error(red("Program, connection, or wallet not found\n\n"));
      return;
    }
    try {
     // await addToken(this.program, this.connection, this.keypair);
    } catch (error) {
      console.error(red(`Error adding token: ${error}\n\n`));
    }
  };

  removeToken = async () => {
    if (!this.program || !this.connection || !this.wallet || !this.keypair) {
      console.error(red("Program, connection, or wallet not found\n\n"));
      return;
    }
    try {
     // await removeToken(this.program, this.keypair);
    } catch (error) {
      console.error(red(`Error removing token: ${error}\n\n`));
    }
  };

  getProgramBalance = async () => {
    if (!this.connection || !this.program) {
      console.error(red("Connection not found\n\n"));
      return;
    }
    try {
      await getProgramBalance(this.connection, this.program);
    } catch (error) {
      console.error(red("Error getting program balance\n\n"));
    }
  };

  checkSPLTokenBalance = async () => {
    if (!this.program || !this.connection || !this.wallet || !this.keypair) {
      console.error(red("Program, connection, or wallet not found\n\n"));
      return;
    }
    try {
    //  await checkSPLTokenBalance(this.program, this.keypair, this.connection);
    } catch (error) {
      console.error(red(`Error checking SPL token balance: ${error}\n\n`));
    }
  };

  depositSol = async () => {
    if (!this.program || !this.keypair) {
      console.error(red("Program or keypair not found\n\n"));
      return;
    }
    try {
     // await depositSOL(this.program, this.keypair);
    } catch (error) {
      console.error(red(`Error depositing SOL: ${error}\n\n`));
    }
  };

  depositSPLToken = async () => {
    if (!this.program || !this.connection || !this.wallet || !this.keypair) {
      console.error(red("Program, connection, or wallet not found\n\n"));
      return;
    }
    try {
     // await depositSPLToken(this.program, this.keypair, this.connection);
    } catch (error) {
      console.error(red(`Error depositing SPL token: ${error}\n\n`));
    }
  };

  mintSPLToken = async () => {
    if (!this.program || !this.connection || !this.wallet || !this.keypair) {
      console.error(red("Program, connection, or wallet not found\n\n"));
      return;
    }
    try {
     // await mintSPLToken(this.program, this.keypair, this.connection);
    } catch (error) {
      console.error(red(`Error minting SPL token: ${error}\n\n`));
    }
  };

  getSPLTokenVaultBalance = async () => {
    if (!this.program || !this.connection || !this.wallet || !this.keypair) {
      console.error(red("Program, connection, or wallet not found\n\n"));
      return;
    }
    try {
     // await getSPLTokenVaultBalance(
      //   this.program,
      //   this.keypair,
      //   this.connection
      // );
    } catch (error) {
      console.error(red(`Error getting SPL token vault balance: ${error}\n\n`));
    }
  };

  addAdmin = async () => {
    if (!this.program || !this.keypair) {
      console.error(red("Program or keypair not found\n\n"));
      return;
    }
    try {
     // await addAdmin(this.program, this.keypair);
    } catch (error) {
      console.error(red(`Error adding admin: ${error}\n\n`));
    }
  };

  removeAdmin = async () => {
    if (!this.program || !this.keypair) {
      console.error(red("Program or keypair not found\n\n"));
      return;
    }
    try {
     // await removeAdmin(this.program, this.keypair);
    } catch (error) {
      console.error(red(`Error removing admin: ${error}\n\n`));
    }
  };

  pauseProgram = async () => {
    if (!this.program || !this.keypair) {
      console.error(red("Program or keypair not found\n\n"));
      return;
    }
    try {
    // await pauseProgram(this.program, this.keypair);
    } catch (error) {
      console.error(red(`Error pausing program: ${error}\n\n`));
    }
  };

  unpauseProgram = async () => {
    if (!this.program || !this.keypair) {
      console.error(red("Program or keypair not found\n\n"));
      return;
    }
    try {
     // await unpauseProgram(this.program, this.keypair);
    } catch (error) {
      console.error(red(`Error unpausing program: ${error}\n\n`));
    }
  };

  banAccount = async () => {
    if (!this.program || !this.keypair) {
      console.error(red("Program or keypair not found\n\n"));
      return;
    }
    try {
    //  await banAccount(this.program, this.keypair);
    } catch (error) {
      console.error(red(`Error banning account: ${error}\n\n`));
    }
  };

  unbanAccount = async () => {
    if (!this.program || !this.keypair) {
      console.error(red("Program or keypair not found\n\n"));
      return;
    }
    try {
    //  await unbanAccount(this.program, this.keypair);
    } catch (error) {
      console.error(red(`Error unbanning account: ${error}\n\n`));
    }
  };

  // withdrawNative = async () => {
  //   if (!this.program || !this.keypair) {
  //     console.error(red("Program or keypair not found\n\n"));
  //     return;
  //   }
  //   try {
  //     await withdrawNative(this.program, this.keypair);
  //   } catch (error) {
  //     console.error(red(`Error withdrawing native: ${error}\n\n`));
  //   }
  // };

  // withdrawToken = async () => {
  //   if (!this.program || !this.keypair || !this.connection) {
  //     console.error(red("Program or keypair not found\n\n"));
  //     return;
  //   }
  //   try {
  //     await withdrawToken(this.program, this.keypair, this.connection);
  //   } catch (error) {
  //     console.error(red(`Error withdrawing token: ${error}\n\n`));
  //   }
  // };

  initialize = async () => {
    if (!this.program || !this.keypair) {
      console.error(red("Program or keypair not found\n\n"));
      return;
    }
    try {
      console.log
      await init(this.program, this.keypair);
    } catch (error) {
      console.error(red(`Error initializing: ${error}\n\n`));
    }
  };

  setBridgeAddress = async () => {
    if (!this.program || !this.keypair) {
      console.error(red("Program or keypair not found\n\n"));
      return;
    }
    try {
    //  await setBridgeAddress(this.program, this.keypair);
    } catch (error) {
      console.error(red(`Error setting bridge: ${error}\n\n`));
    }
  };
}

export default new Interactor();
