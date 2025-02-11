const path = require("path");
const programDir = path.join(__dirname, "programs", "portfolio-bridge");
const idlDir = path.join(__dirname, "target", "idl");
const sdkDir = path.join(__dirname, "generated");
const binaryInstallDir = path.join(__dirname, ".crates");

module.exports = {
  idlGenerator: "anchor",
  programName: "portfolio_bridge",
  programId: "JAP9nCPz8FSQE5ZQY16yhxq1BMbseJnbMViAAtQWAsSN",
  idlDir,
  sdkDir,
  binaryInstallDir,
  programDir,
};
