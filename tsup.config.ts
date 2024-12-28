import { defineConfig } from "tsup";

export default defineConfig({
  entry: ["fetcher/src/index.ts"],

  outDir: "dist/fetcher",
  format: ["cjs", "esm"],

  treeshake: true,
  splitting: false,

  sourcemap: true,
  minify: "terser",
  clean: true,
  dts: true,
});
