# Kindred AidTrail: Deploy Smart Contract to Stellar Testnet (PowerShell)
$ErrorActionPreference = "Stop"

Write-Host "==========================================================" -ForegroundColor Cyan
Write-Host "Kindred AidTrail: Deploy Smart Contract to Stellar Testnet" -ForegroundColor Cyan
Write-Host "==========================================================" -ForegroundColor Cyan

$Network = "testnet"
$RpcUrl = "https://soroban-testnet.stellar.org"
$NetworkPassphrase = "Test SDF Network ; September 2015"

# 1. Identity generation & funding via Friendbot
Write-Host "[1/6] Ensuring deployer identity exists on testnet..." -ForegroundColor Yellow
try {
    stellar keys generate deployer --network $Network --fund
} catch {
    Write-Host "Deployer key exists or friendbot funded: $($_.Exception.Message)"
}
$DeployerAddr = (stellar keys address deployer).Trim()
Write-Host "Deployer Address: $DeployerAddr" -ForegroundColor Green

# 2. Build WASM contract
Write-Host "[2/6] Building contract WASM..." -ForegroundColor Yellow
try {
    stellar contract build
} catch {
    cargo build --target wasm32-unknown-unknown --release
}

$WasmPath = "target/wasm32v1-none/release/aidtrail_contracts.wasm"
if (-not (Test-Path $WasmPath)) {
    $WasmPath = "target/wasm32-unknown-unknown/release/aidtrail_contracts.wasm"
}

if (-not (Test-Path $WasmPath)) {
    Write-Error "Error: Contract WASM not found at $WasmPath"
    exit 1
}
Write-Host "WASM ready at: $WasmPath" -ForegroundColor Green

# 3. Deploy contract
Write-Host "[3/6] Deploying contract to testnet..." -ForegroundColor Yellow
$ContractId = (stellar contract deploy `
    --wasm $WasmPath `
    --source deployer `
    --network $Network `
    --alias aidtrail).Trim()

Write-Host "Contract deployed: $ContractId" -ForegroundColor Green

# 4. Initialize contract
Write-Host "[4/6] Initializing AidTrail contract with admin..." -ForegroundColor Yellow
stellar contract invoke `
    --id aidtrail `
    --source deployer `
    --network $Network `
    -- initialize `
    --admin $DeployerAddr

# 5. Resolve Testnet Asset Contract IDs
Write-Host "[5/6] Resolving native XLM token ID..." -ForegroundColor Yellow
$NativeXlm = (stellar contract id asset --asset native --network $Network).Trim()
Write-Host "Native XLM Asset Contract: $NativeXlm" -ForegroundColor Green

# 6. Save deployment metadata
Write-Host "[6/6] Writing deployment details to deployments/testnet.json..." -ForegroundColor Yellow
if (-not (Test-Path "deployments")) {
    New-Item -ItemType Directory -Path "deployments" | Out-Null
}

$DeploymentData = @{
    network = $Network
    rpcUrl = $RpcUrl
    networkPassphrase = $NetworkPassphrase
    contractId = $ContractId
    contractAlias = "aidtrail"
    admin = $DeployerAddr
    tokens = @{
        native = $NativeXlm
    }
    wasmPath = $WasmPath
    deployedAt = (Get-Date -AsUTC -Format "yyyy-MM-ddTHH:mm:ssZ")
}

$DeploymentData | ConvertTo-Json -Depth 4 | Set-Content "deployments/testnet.json"
Write-Host "Deployment complete! Saved to deployments/testnet.json" -ForegroundColor Green
