export enum Commands {
  Initialize = "1. Initialize",
  SetBridgeAddress = "2. Set Bridge Address",
  // Wallet & Balance
  CreateWallet = "1.  Create Wallet",
  SetActiveWallet = "2.  Set Active Wallet",
  PrintWallet = "3.  Print Wallet Address",
  ShowBalance = "4.  Show Balance of active wallet",
  Airdrop = "5.  Airdrop SOL",
  ProgramBalance = "6.  Get SOL Vault Balance",

  // Token operations
  ListSupported = "1.  List Supported Tokens",
  TokenDetails = "2.  Get Token Details",
  AddToken = "3.  Add Token (only admin)",
  RemoveToken = "4. Remove Token (only admin and paused)",
  MintSPLToken = "5. Mint SPL token (only admin)",
  GetSPLTokenBalance = "6. Get SPL token balance of active wallet",
  GetSPLTokenVaultBalance = "7. Get program SPL token balance",

  // Deposits
  DepositSol = "1. Deposit SOL (only unpaused)",
  DepositSPL = "2. Deposit SPL token (only unpaused)",

  // Account management
  AddAdmin = "1. Add Admin",
  RemoveAdmin = "2. Remove Admin",
  BanAccount = "3. Ban Account",
  UnbanAccount = "4. Unban Account",

  // Program state
  PauseProgram = "1. Pause Program",
  UnpauseProgram = "2. Unpause Program",

  // Withdraws
  WithdrawNative = "1. Withdraw Native",
  WithdrawToken = "2. Withdraw SPL token",
}

enum Sections {
  initialize = "0. Initialize",
  walletAndBalance = "1. Wallet & Balance",
  tokenOptions = "2. Token operations",
  accountManagement = "3. Account management (admin only)",
  programState = "4. Program state (admin only)",
  deposits = "5. Deposits",
  withdraws = "6. Withdraws",
  exit = "7. Exit",
}

// First, let's create a mapping of sections to their commands
const sectionCommands = {
  [Sections.initialize]: [Commands.Initialize, Commands.SetBridgeAddress],
  [Sections.walletAndBalance]: [
    Commands.CreateWallet,
    Commands.SetActiveWallet,
    Commands.PrintWallet,
    Commands.ShowBalance,
    Commands.Airdrop,
    Commands.ProgramBalance,
  ],
  [Sections.tokenOptions]: [
    Commands.ListSupported,
    Commands.TokenDetails,
    Commands.AddToken,
    Commands.RemoveToken,
    Commands.MintSPLToken,
    Commands.GetSPLTokenBalance,
    Commands.GetSPLTokenVaultBalance,
  ],
  [Sections.deposits]: [Commands.DepositSol, Commands.DepositSPL],
  [Sections.accountManagement]: [
    Commands.AddAdmin,
    Commands.RemoveAdmin,
    Commands.BanAccount,
    Commands.UnbanAccount,
  ],
  [Sections.programState]: [Commands.PauseProgram, Commands.UnpauseProgram],
  [Sections.withdraws]: [Commands.WithdrawNative, Commands.WithdrawToken],
  [Sections.exit]: [],
};

export const printSections = () => {
  console.log("================================================");
  Object.values(Sections).forEach((section) => {
    console.log(section);
  });
  console.log("================================================");
};

export const printCommands = (sectionNumber: string) => {
  console.log("================================================");
  const section = Object.values(Sections).find((s) =>
    s.startsWith(sectionNumber + ".")
  );
  if (!section) {
    console.log("Invalid section number");
    return;
  }

  console.log(`== ${section} ==`);
  sectionCommands[section].forEach((cmd) => console.log(cmd));
  console.log("================================================");
};
