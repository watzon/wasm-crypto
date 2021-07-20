#!/bin/sh

set -eux

cargo build --target wasm32-unknown-unknown --release
wasm-bindgen --out-dir pkg --out-name mod --target deno ${CARGO_TARGET_DIR:-target}/wasm32-unknown-unknown/release/wasm_crypto.wasm