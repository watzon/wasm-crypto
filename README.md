# WASM Crypto

Various Rust crypto libraries wrapped in WASM for use in the browser.

# Installation

Instructions coming soon...

# Building

To build this you will need [rust]() and [rustwasmc](https://www.secondstate.io/articles/rustwasmc/). For now `rustwasmc` has the rust version pegged at 1.50.0, so you may need to run `rustup override set 1.50.0` to get the target to build successfully. Once all dependencies are met, just run `rustwasmc build --target deno` in the project root. It will overwrite the files in the `pkg` directory.