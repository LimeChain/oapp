import { getUserInput } from "./utils";
import { printCommands, printSections } from "./command";
import Interactor from "./interactor";
import { green } from "kleur";
import { red } from "kleur";

const exitNum = 7;

const main = async () => {
  while (true) {
    console.clear();
    printSections();
    const sectionCommand = await getUserInput("Enter section: \n");
    console.clear();

    if (Number(sectionCommand) === exitNum) {
      console.log(green("Exiting..."));
      process.exit(0);
    }
    printCommands(sectionCommand);
    const command_str = await getUserInput("Enter command: \n");
    const command = Number(command_str);
    switch (sectionCommand) {
      case "0": // Initialize
        if (command_str === "") {
          continue;
        }
        switch (command) {
          case 1:
            await Interactor.initialize();
            break;
          case 2:
            await Interactor.setBridgeAddress();
            break;
          default:
            console.error(red("\n\nInvalid command!\n\n"));
        }
        break;
      case "1": // Wallet & Balance
        if (command_str === "") {
          continue;
        }
        switch (command) {
          case 1:
            await Interactor.createWallet();
            break;
          case 2:
            await Interactor.loadWallet();
            break;
          case 3:
            await Interactor.printWallet();
            break;
          case 4:
            await Interactor.showBalance();
            break;
          case 5:
            await Interactor.requestAirdrop();
            break;
          case 6:
            await Interactor.getProgramBalance();
            break;
          default:
            console.error(red("\n\nInvalid command!\n\n"));
        }
        break;
      case "2": // Token operations
        if (command_str === "") {
          continue;
        }
        switch (command) {
          case 1:
            await Interactor.getTokenList();
            break;
          case 2:
            await Interactor.getTokenDetails();
            break;
          case 3:
            await Interactor.addToken();
            break;
          case 4:
            await Interactor.removeToken();
            break;
          case 5:
            await Interactor.mintSPLToken();
            break;
          case 6:
            await Interactor.checkSPLTokenBalance();
            break;
          case 7:
            await Interactor.getSPLTokenVaultBalance();
            break;
          default:
            console.error(red("\n\nInvalid command!\n\n"));
        }
        break;
      case "3": // Account management
        if (command_str === "") {
          continue;
        }
        switch (command) {
          case 1:
            await Interactor.addAdmin();
            break;
          case 2:
            await Interactor.removeAdmin();
            break;
          case 3:
            await Interactor.banAccount();
            break;
          case 4:
            await Interactor.unbanAccount();
            break;
          default:
            console.error(red("\n\nInvalid command!\n\n"));
        }
        break;
      case "4": // Program state
        if (command_str === "") {
          continue;
        }
        switch (command) {
          case 1:
            await Interactor.pauseProgram();
            break;
          case 2:
            await Interactor.unpauseProgram();
            break;
          default:
            console.error(red("\n\nInvalid command!\n\n"));
        }
        break;
      case "5": // Deposits
        if (command_str === "") {
          continue;
        }
        switch (command) {
          case 1:
            await Interactor.depositSol();
            break;
          case 2:
            await Interactor.depositSPLToken();
            break;
          default:
            console.error(red("\n\nInvalid command!\n\n"));
        }
        break;
      case "6": // Withdraws
        if (command_str === "") {
          continue;
        }
        switch (command) {
          case 1:
            // await Interactor.withdrawNative();
            break;
          case 2:
            //await Interactor.withdrawToken();
            break;
          default:
            console.error(red("\n\nInvalid command!\n\n"));
        }
        break;
      default:
        console.error(red("\n\nInvalid section!\n\n"));
    }
    await getUserInput("Press any key to continue...");
  }
};

main();
