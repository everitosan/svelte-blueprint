const ffi = require("ffi-napi")
const path = require('path')

const libLocation = process.platform === "win32" ? "./blueprint/target/release/blueprintlib" : "./blueprint/target/release/libblueprintlib"
const libraryName = path.resolve(__dirname, libLocation)

const lib = ffi.Library(libraryName, {
  blueprint: ["string", ["string", "string", "string"]]
})

function entry(source, destination, template = "svelte-blueprint/templates/Blueprint.svelte") {
  const rawRes = lib.blueprint(source, destination, template)
  return rawRes.split("|")
}

module.exports = {
  document: entry
}