# WebSnap

WebSnap is a command-line tool for capturing screenshots of websites. It provides two implementations: a Rust-based version that uses Google Chrome in headless mode to capture screenshots, and a WebAssembly (WASM) version that serves as a placeholder for browser-based screenshot functionality. The tool allows customization of screenshot dimensions, user-agent strings, output formats, and more.

## Features
- Capture website screenshots with customizable width, height, and user-agent.
- Support for output formats: WebP, PNG, JPG.
- Debug mode for detailed logging.
- Command-line interface with flexible arguments.
- WASM integration for potential browser-based usage (placeholder implementation).
- Sleep time configuration to handle dynamic content loading.

## Prerequisites
- **Rust Implementation**:
  - Rust toolchain (`cargo`, `rustc`).
  - Google Chrome (stable version) installed.
  - Dependencies: `clap`, `chrono`, `serde`, `md5`.
- **WASM Implementation**:
  - `wasm-pack` for building WebAssembly.
  - Node.js and npm for JavaScript integration.
  - A modern browser for testing WASM.

## Installation

### Rust Version
1. Clone the repository:
   ```bash
   git clone https://github.com/canmi21/websnap.git
   cd websnap
   ```
2. Install dependencies and build:
   ```bash
   cargo build --release
   ```
3. The binary will be available at `target/release/websnap`.

### WASM Version
1. Install `wasm-pack`:
   ```bash
   cargo install wasm-pack
   ```
2. Build the WASM module:
   ```bash
   wasm-pack build --target web
   ```
3. This generates a `pkg` directory with the WASM module and JavaScript glue code.

## Usage

### Rust Version
Run the tool with the following command:
```bash
websnap --target <URL> [OPTIONS]
```

#### Options
- `-o, --output <FILE>`: Output file or path for the screenshot.
- `-d, --debug`: Enable debug output.
- `-h, --height <PIXELS>`: Screenshot height (default: 1080).
- `-w, --width <PIXELS>`: Screenshot width (default: 1920).
- `-s, --sleep <SECONDS>`: Sleep time before capturing (default: 0).
- `-u, --ua <STRING>`: Custom User-Agent string.
- `-t, --target <URL>`: Target URL to capture (e.g., `https://example.com`).
- `-f, --format <FORMAT>`: Output format (`png`, `webp`, `jpg`).
- `--help`: Print help information.

#### Example
Capture a screenshot of `https://example.com` with a custom size and output format:
```bash
websnap -t https://example.com -o screenshot.png -w 1280 -h 720 -f png -d
```

### WASM Version
The WASM version is a placeholder and logs parameters to the console. To integrate it into a web project:

1. Create an HTML file (e.g., `index.html`):
   ```html
   <!DOCTYPE html>
   <html>
   <head>
       <title>WebSnap WASM</title>
   </head>
   <body>
       <script type="module">
           import init, { capture_screenshot } from './pkg/websnap.js';

           async function run() {
               await init();
               capture_screenshot(
                   "https://example.com",
                   "screenshot.png",
                   1920,
                   1080,
                   "Mozilla/5.0",
                   2,
                   true
               );
           }

           run();
       </script>
   </body>
   </html>
   ```
2. Copy the `pkg` directory to your web project.
3. Serve the project using a local server (e.g., `npx serve`).
4. Open the browser console to see the logged output.

**Note**: The WASM version requires a JavaScript-based screenshot implementation (e.g., using Puppeteer or canvas) to be fully functional.

## Compilation and Packaging

### Rust Compilation
- Build the Rust binary:
  ```bash
  cargo build --release
  ```
- Run tests (if any):
  ```bash
  cargo test
  ```
- Package the binary for distribution:
  ```bash
  cargo package
  ```

### WASM Compilation
- Build the WASM module:
  ```bash
  wasm-pack build --target web
  ```
- Test the WASM module in a browser:
  ```bash
  npx serve
  ```
- Package for npm:
  ```bash
  wasm-pack pack
  ```

## npm Integration
To publish the WASM module to npm:

1. Update the `pkg/package.json` with your project details (e.g., name, version).
2. Publish to npm:
   ```bash
   cd pkg
   npm publish
   ```
3. Install in a JavaScript project:
   ```bash
   npm install websnap
   ```
4. Import and use in your JavaScript code as shown in the WASM usage example.

## Project Structure
```
websnap/
├── src/
│   ├── main.rs          # Rust CLI implementation
│   └── lib.rs           # WASM implementation
├── Cargo.toml           # Rust dependencies and metadata
├── README.md            # This file
└── pkg/                 # WASM output (after wasm-pack build)
```

## Limitations
- The Rust version requires Google Chrome to be installed and only works on systems supporting Chrome's headless mode.
- The WASM version is a placeholder and does not capture screenshots; it requires a JavaScript-based implementation for full functionality.
- No support for capturing dynamic content requiring user interaction.

## Contributing
Contributions are welcome! Please open an issue or submit a pull request on GitHub.

## License
MIT License. See `LICENSE` for details.