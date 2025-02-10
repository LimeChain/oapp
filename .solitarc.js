const path = require("path");
const programDir = path.join(__dirname, "programs", "portfolio-bridge");
const idlDir = path.join(__dirname, "idl");
const sdkDir = path.join(__dirname, "generated");
const binaryInstallDir = path.join(__dirname, ".crates");

module.exports = {
  idlGenerator: "anchor",
  programName: "portfolio_bridge",
  programId: "9Fmenbf7Qti4sG3hQWwifpAvGArtqtK9N96jdN19MX3u",
  idlDir,
  sdkDir,
  binaryInstallDir,
  programDir,
};
