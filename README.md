# 🖨️ `@agsolutions-at/printers`

[![npm version](https://img.shields.io/npm/v/@agsolutions-at/printers.svg)](https://www.npmjs.com/package/@agsolutions-at/printers)
[![npm downloads](https://img.shields.io/npm/dm/@agsolutions-at/printers.svg)](https://www.npmjs.com/package/@agsolutions-at/printers)
[![license](https://img.shields.io/npm/l/@agsolutions-at/printers.svg)](./LICENSE)
[![node](https://img.shields.io/node/v/@agsolutions-at/printers)](https://nodejs.org)
[![platforms](https://img.shields.io/badge/platforms-macOS%20%7C%20Windows-blue)](#)
[![CI](https://github.com/agsolutions-at/printers/actions/workflows/CI.yml/badge.svg)](https://github.com/agsolutions-at/printers/actions/workflows/CI.yml)

**`@agsolutions-at/printers`** is a high-performance, Rust-powered replacement for outdated native printer libraries in Node.js. Built on top of [
`rust-printers`](https://github.com/talesluna/rust-printers), it provides seamless bindings via [`napi-rs`](https://napi.rs/), supporting fast and
reliable printer interactions in Node.js and Electron applications.

> ✅ Prebuilt native binaries included — no need to build from source for most users.

## ✨ Features

- ⚡ **Powered by Rust** — high performance, memory-safe.
- 🔌 **Native Node.js bindings** via [`napi-rs`](https://napi.rs/).
- 🧩 **Electron-friendly** — includes prebuilt binaries, plug-and-play.
- 🖥️ **Cross-platform aware** — currently supports **macOS** and **Windows**.
- 💡 **Easy-to-use API** for interacting with system printers.

> ℹ️ Want Linux support? PRs are welcome!

## 📦 Installation

Install using your preferred package manager:

```bash
# npm
npm install @agsolutions-at/printers

# yarn
yarn add @agsolutions-at/printers

# pnpm
pnpm add @agsolutions-at/printers
```

> 🧱 No native build step needed — prebuilt binaries are downloaded for your platform automatically.

## 🚀 Quick Start

Here's a basic example to get up and running:

```ts
import { getPrinters, print, getActiveJobs, getJobHistory } from '@agsolutions-at/printers';

const printers = getPrinters();
console.log('Available printers:', printers);

const buffer = new TextEncoder().encode('Hello, printers!');
print(printers[0].name, buffer, 'My Test Job');

const jobs = getActiveJobs(printers[0].name);
console.log('Active jobs:', jobs);
```

> 🔍 All bindings mirror the native Rust API. Check [index.d.ts](./index.d.ts) for full typings and usage.

## 🧪 CLI Testing

This repo includes a command-line utility: [`printer-cli.mjs`](./printer-cli.mjs), which makes it easy to test the API from the terminal.

### 🏃 Run the CLI:

```bash
node printer-cli.mjs
```
### 💡 Features:
- List available printers
- Select and print text
- Print a file
- View active jobs
- View job history

> Perfect for debugging or quick testing without writing your own app.

## 🛠 Building from Source

If you prefer to build locally:

1. **Clone the repository**:
   ```bash
   git clone https://github.com/agsolutions-at/printers.git
   cd printers
   ```

2. **Install dependencies**:
   ```bash
   yarn install
   ```

3. **Build the native module**:
   ```bash
   yarn build
   ```

> 🛠 Prerequisites: Rust toolchain (`rustc`, `cargo`) and Node.js installed.

## 🤝 Contributing

We welcome contributions of all kinds — bug reports, feature requests, docs, and PRs!

👉 Submit an issue or pull request on [GitHub](https://github.com/agsolutions-at/printers).

## 📄 License

MIT © [agsolutions GmbH](https://agsolutions.at)  
See [LICENSE](./LICENSE) for full details.

