# DEMLE Smart Contracts

Smart contracts for DEMLE (Decentralized Machine Learning Engine) - a cryptocurrency that rewards FP8 ML mining computations.

## Quick Start (Fixed)

### 1. 🚀 Complete Setup (Recommended)
```bash
# Start Hardhat node (keep running in one terminal)
npx hardhat node

# In another terminal, run the complete setup
npm run setup-demo
```

This will:
- Deploy a fresh DEMLE contract
- Generate a real-time dashboard (demle-dashboard.html)
- Show you the exact commands to start mining
- Keep the network running for testing

### 2. ⛏️ Start Mining
Use the command shown in the setup output:
```bash
cargo run --bin demle-miner -- --contract <DEPLOYED_ADDRESS> --rpc-url http://localhost:8545
```

### 3. 📊 Monitor Activity
- Open `demle-dashboard.html` in your browser for real-time updates
- Or run: `npm run check-live-balances`

## Available Scripts

### Setup & Deployment
- `npm run setup-demo` - Complete environment setup with dashboard
- `npm run deploy` - Deploy contract only
- `npm run node` - Start Hardhat local network

### Monitoring
- `npm run check-live-balances` - Check all active contracts and balances
- `npm run realtime-demo` - Legacy realtime demo (use setup-demo instead)

### Development
- `npm run compile` - Compile contracts
- `npm run test` - Run tests
- `npm run clean` - Clean build artifacts

## Fixed Issues

✅ **Contract Address Mismatch**: Setup now deploys and provides correct addresses
✅ **Command Line Flags**: Fixed `--rpc` to `--rpc-url` in all instructions  
✅ **Contract Detection**: Scripts now automatically find deployed contracts
✅ **Required Parameters**: Miner now requires contract address (no hardcoded defaults)

## Troubleshooting

### "Contract not found" or "Bad data" errors
```bash
# Check what contracts are actually deployed
npm run check-live-balances

# If no contracts found, run fresh setup
npm run setup-demo
```

### Node.js version warnings
The warnings about Node.js v23.10.0 are non-critical but you can use Node v18 or v20 if preferred.

### Dashboard not updating
1. Ensure Hardhat node is running (`npx hardhat node`)
2. Check that the contract address in the dashboard matches your deployed contract
3. Verify the miner is actually submitting transactions

## Architecture

- **DEMLE.sol**: ERC20 token with mining rewards for ML computations
- **Mining Reward**: 100 DEMLE tokens per successful proof submission
- **Max Supply**: 21 million DEMLE tokens
- **Proof System**: Simplified for demo (verifies FP8 ML computation proofs)

## Development Workflow

1. Run `npx hardhat node` to start local network
2. Deploy contracts with `npm run deploy` or `npm run setup-demo`
3. Start mining with the Rust miner using the deployed contract address
4. Monitor activity via dashboard or CLI tools

The dashboard updates every 3 seconds and shows real-time mining activity, token distribution, and miner statistics. 