const path = require("path");
const programDir = path.join(__dirname, "programs", "portfolio-bridge");
const idlDir = path.join(__dirname, "target", "idl");
const sdkDir = path.join(__dirname, "generated");
const binaryInstallDir = path.join(__dirname, ".crates");

module.exports = {
  idlGenerator: "anchor",
  programName: "portfolio_bridge",
  programId: "DD12vMyLdwszDCAzLhsUPwBmzJXv611dUCPhqwpZQYG4",
  idlDir,
  sdkDir,
  binaryInstallDir,
  programDir,
};
