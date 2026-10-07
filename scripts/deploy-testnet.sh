#!/usr/bin/env bash
set -euo pipefail

echo "=========================================================="
echo "Kindred AidTrail: Deploy Smart Contract to Stellar Testnet"
echo "=========================================================="

NETWORK="testnet"
RPC_URL="https://soroban-testnet.stellar.org"
NETWORK_PASSPHRASE="Test SDF Network ; September 2015"

# 1. Identity generation & funding via Friendbot
echo "[1/6] Ensuring deployer identity exists on testnet..."
stellar keys generate deployer --network "$NETWORK" --fund || true
DEPLOYER_ADDR=$(stellar keys address deployer)
echo "Deployer Address: $DEPLOYER_ADDR"

# 2. Build WASM contract
echo "[2/6] Building contract WASM..."
if command -v stellar &> /dev/null; then
  stellar contract build
else
  cargo build --target wasm32-unknown-unknown --release
fi

WASM_PATH="target/wasm32v1-none/release/aidtrail_contracts.wasm"
if [ ! -f "$WASM_PATH" ]; then
  WASM_PATH="target/wasm32-unknown-unknown/release/aidtrail_contracts.wasm"
fi

if [ ! -f "$WASM_PATH" ]; then
  echo "Error: WASM file not found at $WASM_PATH"
  exit 1
fi
echo "WASM ready at: $WASM_PATH"

# 3. Deploy contract
echo "[3/6] Deploying contract to testnet..."
CONTRACT_ID=$(stellar contract deploy \
  --wasm "$WASM_PATH" \
  --source deployer \
  --network "$NETWORK" \
  --alias aidtrail)

echo "Contract deployed successfully: $CONTRACT_ID"

# 4. Initialize contract
echo "[4/6] Initializing AidTrail contract with admin..."
stellar contract invoke \
  --id aidtrail \
  --source deployer \
  --network "$NETWORK" \
  -- initialize \
  --admin "$DEPLOYER_ADDR"

# 5. Resolve Testnet Asset Contract IDs
echo "[5/6] Resolving native XLM and Circle USDC token IDs..."
NATIVE_XLM=$(stellar contract id asset --asset native --network "$NETWORK")
USDC_TESTNET=$(stellar contract id asset --network "$NETWORK" \
  --asset "USDC:GBBD47IF6LWK7P7MDEVSCWR7DPUWV3NY3DTQEVFL4NAT4AQH3ZLLFLA5" || echo "")

echo "Native XLM Asset Contract: $NATIVE_XLM"

# 6. Save deployment metadata
echo "[6/6] Writing deployment details to deployments/testnet.json..."
mkdir -p deployments
cat <<EOF > deployments/testnet.json
{
  "network": "$NETWORK",
  "rpcUrl": "$RPC_URL",
  "networkPassphrase": "$NETWORK_PASSPHRASE",
  "contractId": "$CONTRACT_ID",
  "contractAlias": "aidtrail",
  "admin": "$DEPLOYER_ADDR",
  "tokens": {
    "native": "$NATIVE_XLM",
    "usdc": "$USDC_TESTNET"
  },
  "wasmPath": "$WASM_PATH",
  "deployedAt": "$(date -u +"%Y-%m-%dT%H:%M:%SZ")"
}
EOF

echo "Deployment complete! Saved to deployments/testnet.json"
