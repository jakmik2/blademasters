#!/usr/bin/env bash

PROJECT_NAME="combative-survivors"

# Build
cargo build --target wasm32-unknown-unknown --release --no-default-features

# Generate bindgen outputs
mkdir -p out
mkdir -p out/assets

wasm-bindgen --no-typescript --target web \
    --out-dir ./out/ \
   --out-name "$PROJECT_NAME" \
   ./target/wasm32-unknown-unknown/release/$PROJECT_NAME.wasm
