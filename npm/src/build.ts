#!/usr/bin/env node   
import { mkdir, readFile, writeFile, rm } from "node:fs/promises";
import { spawnSync } from "node:child_process";
import { join } from "node:path";
import { minify } from "terser";

const UTILITIES_PACKAGE_NAME = "@literate.ink/utilities";

const cwd = process.cwd();
const pkg = JSON.parse(await readFile(join(cwd, "package.json"), "utf8")) as {
  /** Needs to be the same name as the `Cargo.toml` file */
  name: string
}

// We build the WASM module inside `js-prepkg` folder.
spawnSync('wasm-pack', ["build",
  "--out-dir", "js-prepkg",
  "--target", "web",
  "--release"
], {
  stdio: "inherit",
  cwd
});

await rm(join(cwd, "js-dist"), { force: true, recursive: true });
await mkdir(join(cwd, "js-dist"));

const pathFromPKG = (name: string): string => join(cwd, "js-prepkg", name);
const writeToDIST = (name: string, content: string): Promise<void> => writeFile(join(cwd, "js-dist", name), content, "utf8");

const removeFromUntil = (content: string, startsWith: string, endsWith: string): string => {
  const start = content.indexOf(startsWith);
  const end = content.indexOf(endsWith, start) + endsWith.length;
  return content.slice(0, start) + content.slice(end);
}

const WASM = await readFile(pathFromPKG(`${pkg.name}_bg.wasm`));

{ // Process the JS file.
  let content = await readFile(pathFromPKG(`${pkg.name}.js`), "utf8");
  
  // Add an import to utilities.
  content = `const { defaultFetcher: utils__defaultFetcher } = require(${JSON.stringify(UTILITIES_PACKAGE_NAME)});\n` + content;
  console.info("[JS]: Added utilities import (using require)");

  // Add default fetcher to the fetcher parameter (to make it optional)
  content = content.replace(
    // Since fetcher parameter is always the last, we can match it that way. 
    /, fetcher\) {/g,
    `, fetcher = utils__defaultFetcher) {`
  );
  console.info("[JS]: Added default fetcher to 'fetcher' parameters.");

  content = content.replace("export { initSync }", "");
  content = content.replace("export default __wbg_init;", "");
  console.info("[JS]: Removed old exports.");

  content = removeFromUntil(content, "async function __wbg_init", "return __wbg_finalize_init(instance, module);\n}");
  content = content.replace("__wbg_init.__wbindgen_wasm_module = module;", "");
  console.info("[JS]: Removed '__wbg_init' function.");

  const exports: string[] = [];

  content = content.replace(/export class (\w+)/g, (_, match) => {
    console.log("[JS]: Found class:", match);

    exports.push(match);
    return `class ${match}`;  
  });

  content = content.replace(/export function (\w+)/g, (_, match) => {
    console.log("[JS]: Found function:", match);

    exports.push(match);
    return `function ${match}`;
  });

  content = content.replace(/export const (\w+)/g, (_, match) => {
    console.log("[JS]: Found constant:", match);

    exports.push(match);
    return `const ${match}`;
  });

  content += exports.map((name) => `exports.${name} = ${name};`).join("\n");
  console.info("[JS]: Rewrote exports.");

  content += `
const __code = ${JSON.stringify("data:application/wasm;base64," + WASM.toString("base64"))};

function sync_read () {
  var pos = __code;
  
  try {
    var binary = atob_polyfill(pos.slice(29)) // "data:application/wasm;base64,".length
    var output = new Uint8Array(binary.length);
    
    for (pos = 0; pos < binary.length; ++pos)
      output[pos] = binary.charCodeAt(pos);
    
    return output;
  }
  catch {
    throw Error("Converting base64 string to bytes failed.");
  }
}

var atob_polyfill = "function" == typeof atob ? atob : function(a){
  var chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/="
  var b = "";
  var c = 0;
  a = a.replace(/[^A-Za-z0-9\\+\\/=]/g,"");
  
  do {
    var d=chars.indexOf(a.charAt(c++));
    var e=chars.indexOf(a.charAt(c++));
    var f=chars.indexOf(a.charAt(c++));
    var g=chars.indexOf(a.charAt(c++));
    
    d = d<<2|e>>4;
    e = (e & 15) <<4|f>>2;
    
    var k=(f&3)<<6|g;
    b += String.fromCharCode(d);
    
    64 !== f && (b+=String.fromCharCode(e));
    64 !== g && (b+=String.fromCharCode(k))
  } while (c < a.length);
  
  return b;
}

const load_sync = () => void initSync(sync_read());
exports.load_sync = load_sync;

exports.load_async = async function () {
  if (wasm !== undefined) return;
  const imports = __wbg_get_imports();

  if (typeof fetch !== "function")
    return load_sync();

  const binary = await fetch(__code, { credentials: "same-origin" });

  __wbg_init_memory(imports);

  const { instance, module } = await __wbg_load(binary, imports);
  __wbg_finalize_init(instance, module);
}
  `;
  console.info("[JS]: Added 'load_async' and 'load_sync' functions and copied WASM file into the file.");

  content = (await minify(content)).code || "";
  console.info("[JS]: Minified !");

  await writeToDIST("index.js", content);
  console.info("[JS]: Wrote file.");
}

{ // Process the D.TS file.
  let content = await readFile(pathFromPKG(`${pkg.name}.d.ts`), "utf8");

  // Add an import to utilities.
  content = `import type { Fetcher } from '${UTILITIES_PACKAGE_NAME}';\n` + content;
  console.info("[D.TS]: Added utilities import.");

  // Make every fetcher optional by adding a default fetcher (and the type)
  content = content.replace(
    /fetcher: Function\)/g,
    `fetcher?: Fetcher)`
  );
  console.info("[D.TS]: Typed 'fetcher' and made it optional.");

  // Do the same in the JSDoc comments.
  content = content.replace(
    /@param {Function} fetcher/g,
    `@param {Fetcher} [fetcher]`
  );
  console.info("[D.TS]: Typed 'fetcher' and made it optional in JSDoc.");

  content = removeFromUntil(content, "export type InitInput", "Promise<InitOutput>;");
  console.info("[D.TS]: Removed useless types.");

  content += `
/**
 * Load the WASM module asynchronously using "fetch".
 * If "fetch" is not available, it will load synchronously using "load_sync".
 * @returns {Promise<void>}
 */
export function load_async(): Promise<void>;
/**
 * Load the WASM module using "Uint8Array" (synchronously).
 * @returns {void}
 */
export function load_sync(): void;
  `;
  console.info("[D.TS]: Added 'load_async' and 'load_sync' functions.");

  await writeToDIST("index.d.ts", content);
  console.info("[D.TS]: Wrote file.");
}

await rm(join(cwd, "js-prepkg"), { force: true, recursive: true }).catch(() => void 0);

console.info("[INFO]: Done !");
console.info("[INFO]: You can find the bundle in ./js-dist/");
