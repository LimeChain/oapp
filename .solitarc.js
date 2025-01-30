const path = require("path");
const programDir = path.join(__dirname, "programs", "oapp-example");
const idlDir = path.join(__dirname, "idl");
const sdkDir = path.join(__dirname, "generated");
const binaryInstallDir = path.join(__dirname, ".crates");

module.exports = {
  idlGenerator: "anchor",
  programName: "oapp_example",
  programId: "GG9GMa3Y7ow2j9jRgbTusBHc57VUh55G4wfbVskhjkbh",
  idlDir,
  sdkDir,
  binaryInstallDir,
  programDir,
};
