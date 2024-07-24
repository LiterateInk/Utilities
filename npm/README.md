# LiterateInk: `utilities`

A collection of utilities for our libraries to prevent us from writing the same code again and again in multiple repositories that finally gets out of sync.

## Installation

Use the package manager of your choice to install this library.

```bash
pnpm add @literate.ink/utilities
yarn add @literate.ink/utilities
bun add @literate.ink/utilities
npm add @literate.ink/utilities
```

## Usage

```typescript
import {
  type Request,
  type Response,
  type Fetcher,
  defaultFetcher
} from "@literate.ink/utilities";
```

Also provides a `wasm-build` binary to build a project using [`wasm-pack`](https://github.com/rustwasm/wasm-pack) and [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen).

You can use this as your `build` script in `package.json`.

```jsonc
{
  "name": "my-wasm-project", // Should match the name in Cargo.toml
  "scripts": {
    "build": "wasm-build"
  }
}
```

## Documentation

A reference will be available at `https://docs.literate.ink/utilities`.
In the meantime, please use intellisense to read the documentation of each functions.
