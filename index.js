const ffi = require("ffi-napi")
const path = require('path')

const library_name = path.resolve(__dirname, './blueprint/target/release/libblueprintlib');

const lib = ffi.Library(library_name, {
  blueprint: ["string", ["string", "string", "string"]]
})

function entry(source, destination, template = "svelte-blueprint/templates/Blueprint.svelte") {
  const rawRes = lib.blueprint(source, destination, template)
  return rawRes.split("|")
}

module.exports = {
  document: entry
}