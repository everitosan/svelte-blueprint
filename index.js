const ffi = require("ffi-napi")
const path = require('path')
import * as pack from "./package.json"

const library_name = path.resolve(__dirname, './blueprint/target/release/libblueprintlib');

const lib = ffi.Library(library_name, {
  blueprint: ["bool", ["string", "string", "string"]]
})

function entry(source, destination, template = `${pack.name}/templates/Blueprint.svelte`) {
  return lib.blueprint(source, destination, template)
}

module.exports = {
  document: entry
}