# 🖨️ printers

[![npm version](https://img.shields.io/npm/v/@agsolutions-at/printers.svg)](https://www.npmjs.com/package/@agsolutions-at/printers)
[![npm downloads](https://img.shields.io/npm/dm/@agsolutions-at/printers.svg)](https://www.npmjs.com/package/@agsolutions-at/printers)
[![license](https://img.shields.io/npm/l/@agsolutions-at/printers.svg)](./LICENSE)
[![node](https://img.shields.io/node/v/@agsolutions-at/printers)](https://nodejs.org)
[![platforms](https://img.shields.io/badge/platforms-macOS%20%7C%20Windows-blue)](#)
[![CI](https://github.com/agsolutions-at/printers/actions/workflows/CI.yml/badge.svg)](https://github.com/agsolutions-at/printers/actions/workflows/CI.yml)

**`printers`** is a high-performance, Rust-powered replacement for outdated native printer libraries in Node.js. Built on top of [
`rust-printers`](https://github.com/talesluna/rust-printers), it provides seamless bindings via [`napi-rs`](https://napi.rs/), supporting fast and
reliable printer interactions in Node.js and Electron applications.

> ✅ Prebuilt native binaries included — no need to build from source for most users.

## ✨ Features

- ⚡ **Powered by Rust** — high performance, memory-safe.
- 🔌 **Native Node.js bindings** via [`napi-rs`](https://napi.rs/).
- 🧩 **Electron-friendly** — includes prebuilt binaries, plug-and-play.
- 🖥️ **Cross-platform aware** — currently supports **macOS** and **Windows**.
- 💡 **Easy-to-use API** for interacting with system printers.
- 📄 **Native PDF support on macOS** — uses CUPS with native PDF handling on UNIX systems; see [PDF Printing on Windows](#pdf-printing-on-windows).
- 🧾 **Label printer compatible** — works with devices like Rollo and Zebra.

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
import {getPrinters, print, getActiveJobs, getJobHistory} from '@agsolutions-at/printers';

const printers = getPrinters();
console.log('Available printers:', printers);

const buffer = new TextEncoder().encode('Hello, printers!');
const jobId = print(printers[0].name, buffer, 'My Test Job', []);
console.log('Printed with print job ID: ', jobId);

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

## 📄 PDF Printing on Windows

Unlike macOS (which supports native PDF printing via CUPS), **Windows does not natively support raw PDF printing** for most printers.
This means sending a PDF buffer directly may result in garbage output — unless the printer explicitly supports PDF.

To reliably print PDFs on Windows, consider one of the following **workarounds**:

### Use SumatraPDF (Silent PDF Printing)

One of the most reliable ways to print PDFs on Windows is to use [**SumatraPDF**](https://www.sumatrapdfreader.org/free-pdf-reader.html),
a lightweight PDF viewer.

You can invoke it from Node.js using `child_process`:

```ts
import {execFile} from 'child_process';

const sumatraPath = 'C:\\path\\to\\SumatraPDF.exe'; // make sure to use the .exe matching your arch x64, ia32...
const printerName = 'Your Printer Name';
const filePath = 'C:\\path\\to\\your.pdf';

execFile(sumatraPath, ['-print-to', printerName, '-silent', filePath], (err) => {
  if (err) {
    console.error('Printing failed:', err);
  } else {
    console.log('Printed PDF successfully.');
  }
});
```

> ✅ SumatraPDF handles the rendering and sends the job to the printer via the default Windows printing pipeline. Use [
`pdf-to-printer`](https://github.com/artiebits/pdf-to-printer/blob/master/src/print/print.ts) as a reference if you want to wrap SumatraPDF.

### Alternative: Convert PDF to EMF or PS Before Printing

Another approach is to convert the PDF to a more universally printer-friendly format (like **EMF** or **PostScript**) before sending it to the
printer:

- Use **Ghostscript** to convert PDF to `.ps`:
  ```bash
  gswin64c -dNOPAUSE -dBATCH -sDEVICE=ps2write -sOutputFile=output.ps input.pdf
  ```

- Then use Node.js to print the `.ps` file using the `print()` function from this package.

## Using `electron-builder` to bundle the platform-specific native module

When building an Electron app, you may need native modules that are specifically compiled for your target operating system and architecture (like
getting the right key to fit the right lock). The following script helps make sure the correct .node binaries are downloaded just before packaging the
app using electron-builder.

```json
// package.json
{
  ...
  "build": {
    "beforePack": "./beforePack.js",
    "files": [
      ...
      "!**/node_modules/@agsolutions-at/printers-*/**"
    ]
  },
  ...
}
```

Define a `beforePack` hook. Do not include optional dependencies of your build platform.

```typescript
// beforePack.js
import path from 'node:path';
import https from 'node:https';
import fs from 'node:fs';
import printersPackage from './app/node_modules/@agsolutions-at/printers/package.json' with {type: 'json'};
import {fileURLToPath} from 'node:url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const downloadFile = (url, dest, cb) => {
  https
  .get(url, res => {
    // If redirect
    if (res.statusCode >= 300 && res.statusCode < 400 && res.headers.location) {
      return downloadFile(res.headers.location, dest, cb); // follow redirect
    }

    const fileStream = fs.createWriteStream(dest);
    res.pipe(fileStream);

    fileStream.on('finish', () => {
      fileStream.close(cb); // call callback on finish
    });

    res.on('error', err => {
      fs.unlink(dest, () => {
      });
      cb(err.message);
    });
  })
  .on('error', err => {
    cb(err.message);
  });
};

const beforePack = context => {
  const {electronPlatformName, arch} = context;

  let archName;
  switch (arch) {
    case 0:
      archName = 'ia32';
      break;
    case 1:
      archName = 'x64';
      break;
    case 2:
      archName = 'armv7l';
      break;
    case 3:
      archName = 'arm64';
      break;
    case 4:
      archName = 'universal';
      break;
    default:
      throw Error('Unknown arch');
  }

  let downloadUrl;
  if (electronPlatformName === 'win32') {
    downloadUrl = `https://github.com/agsolutions-at/printers/releases/download/v${printersPackage.version}/printers.win32-${archName}-msvc.node`;
  } else {
    // changes this dependening on your needs, in this case we bundle an unversal module.
    downloadUrl = `https://github.com/agsolutions-at/printers/releases/download/v${printersPackage.version}/printers.darwin-universal.node`;
  }
  const nativeModulePath = path.join(
      __dirname,
      'app',
      'node_modules',
      '@agsolutions-at',
      'printers',
      electronPlatformName === 'win32' ? `printers.win32-${archName}-msvc.node` : `printers.darwin-universal.node`
  );
  downloadFile(downloadUrl, nativeModulePath, err => {
    if (err) {
      console.error('Download error:', err);
      process.exit(1);
    } else {
      console.log('Download printers completed');
    }
  });
};

export default beforePack;
```

`beforePack.js`: before your app gets packaged, it sneaks in and places the correct native modules on stage based on the OS and architecture you're
targeting. It detects whether you're building for Windows or macOS, figures out the architecture (x64, arm64, etc.), downloads the correct version
of native .node files from GitHub releases and saves them into the appropriate module folders.

## 🤝 Contributing

We welcome contributions of all kinds — bug reports, feature requests, docs, and PRs!

👉 Submit an issue or pull request on [GitHub](https://github.com/agsolutions-at/printers).

## 📄 License

MIT © [agsolutions GmbH](https://agsolutions.at)  
See [LICENSE](./LICENSE) for full details.

