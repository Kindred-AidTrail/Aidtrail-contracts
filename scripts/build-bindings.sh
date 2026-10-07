#!/usr/bin/env bash
set -euo pipefail

echo "Building Kindred AidTrail WASM and TypeScript Bindings..."

cargo build --target wasm32-unknown-unknown --release

WASM_PATH="target/wasm32-unknown-unknown/release/aidtrail_contracts.wasm"
CONTRACT_ID="CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC"

if command -v stellar &> /dev/null; then
  stellar contract bindings typescript \
    --wasm "$WASM_PATH" \
    --output-dir packages/aidtrail-sdk \
    --contract-id "$CONTRACT_ID" \
    --overwrite
fi

cd packages/aidtrail-sdk
npm install
npm run build
echo "TypeScript SDK built successfully in packages/aidtrail-sdk/dist"
