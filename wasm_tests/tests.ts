import {
  assertEquals,
  assertObjectMatch,
} from "https://deno.land/std@0.100.0/testing/asserts.ts";
import { fromFileUrl } from "https://deno.land/std@0.100.0/path/mod.ts";
import init, { getDocAst } from "../generated/esmdoc.js";

const wasmPath = fromFileUrl(
  new URL("../generated/esmdoc_bg.wasm", import.meta.url).href,
);
const wasm = Deno.readFile(wasmPath);

const loadStart = performance.now();
await init(wasm);

console.log("wasm loading time - %dms", performance.now() - loadStart);

function urlSpecifierResolver(specifier: string, referrer: string): string {
  return new URL(specifier, referrer).href;
}

async function urlSourceCodeLoader(specifier: string): Promise<string> {
  return Deno.readTextFile(fromFileUrl(specifier));
}

Deno.test("typescript no imports", async () => {
  const ast = await getDocAst(
    new URL("../test_fixtures/javascript/no_imports.js", import.meta.url).href,
    urlSpecifierResolver,
    urlSourceCodeLoader,
  );

  assertEquals(ast.length, 1);
  assertObjectMatch(ast[0], {
    jsDoc: "This is an exported variable.\n@type {number}",
    kind: "variable",
    location: {
      col: 0,
      line: 5,
    },
    name: "foo",
    variableDef: {
      kind: "const",
      tsType: {
        kind: "literal",
        repr: "42",
      },
    },
  });
});

// Deno.test("typescript with imports", async () => {
//   const ast = await getDocAst(
//     new URL("../test_fixtures/javascript/with_imports.js", import.meta.url)
//       .href,
//     urlSpecifierResolver,
//     urlSourceCodeLoader,
//   );
//
//   assertEquals(ast.length, 1);
//   assertObjectMatch(ast[0], {
//     jsDoc: "This is an exported variable.",
//     kind: "variable",
//     location: {
//       col: 0,
//       line: 4,
//     },
//     name: "foo",
//     variableDef: {
//       kind: "const",
//       tsType: {
//         keyword: "number",
//         kind: "keyword",
//         repr: "number",
//       },
//     },
//   });
// });
