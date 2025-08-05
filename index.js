const koffi = require("koffi");

const path = require("path");

const libLocation =
  process.platform === "win32"
    ? "./blueprint/target/release/blueprintlib.dll"
    : "./blueprint/target/release/libblueprintlib.so";

const libraryName = path.resolve(__dirname, libLocation);

const lib = koffi.load(libraryName);

const blueprint = lib.func("blueprint", "string", [
  "string",
  "string",
  "string",
]);

function entry(
  source,
  destination,
  template = "svelte-blueprint/templates/Blueprint.svelte"
) {
  try {
    const rawRes = blueprint(source, destination, template);
    return rawRes.split("|");
  } catch (error) {
    console.error("Error calling blueprint function:", error);
    throw error;
  }
}

module.exports = {
  document: entry,
};
