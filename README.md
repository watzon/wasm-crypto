# WASM Crypto

Various Rust crypto libraries wrapped in WASM for use in the browser.

# Installation

Instructions coming soon...

# Building

To build this you will need [rust](https://www.rust-lang.org/tools/install) and [wasm-bindgen-cli](https://rustwasm.github.io/docs/wasm-bindgen/reference/cli.html). You will also need the toolchain `wasm32-unknown-unknown`, which you can install with `rustup target add wasm32-unknown-unknown`.

Once all dependencies are met, just run `./build.sh` in the project root. It will overwrite the files in the `pkg` directory.