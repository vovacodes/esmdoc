const packageJson = await Deno.readTextFile("generated/package.json").then(text => JSON.parse(text))

// Add correct package type: https://nodejs.org/api/packages.html#packages_type
packageJson.type = "module"

// Remove legacy module entrypoint field
delete packageJson.module

// Add "exports" field: https://nodejs.org/api/packages.html#packages_exports
packageJson.exports = {
    ".": "./esmdoc.js",
    "./esmdoc_bg.wasm": "./esmdoc_bg.wasm"
}

packageJson.files = [
    "esmdoc_bg.wasm",
    "esmdoc_bg.wasm.d.ts",
    "esmdoc.js",
    "esmdoc.d.ts"
]

await Deno.writeTextFile("generated/package.json", JSON.stringify(packageJson, null, 4))
