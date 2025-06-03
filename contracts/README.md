# DEMLE Smart Contracts

Smart contracts for the DEMLE (Decentralized Machine Learning Efforts) blockchain project.

## What was fixed

1. **Dependency Conflict**: Removed incompatible `@nomiclabs/hardhat-ethers@^2.2.3` which required ethers v5
2. **Modern Hardhat Setup**: Using `@nomicfoundation/hardhat-toolbox@^4.0.0` which supports ethers v6
3. **TypeScript Configuration**: Added proper `tsconfig.json` for modern Node.js compatibility
4. **Solidity Version**: Updated to ^0.8.20 to match OpenZeppelin v5 requirements

## Quick Start

```bash
# Install dependencies
npm install

# Compile contracts
npx hardhat compile

# Run tests
npx hardhat test

# Deploy locally
npx hardhat run scripts/deploy.ts

# Start local blockchain
npx hardhat node

# Deploy to sepolia (requires .env setup)
npx hardhat run scripts/deploy.ts --network sepolia
```

## Environment Setup

Create a `.env` file with:
```
SEPOLIA_RPC_URL=https://rpc.sepolia.org
PRIVATE_KEY=your_private_key_here
ETHERSCAN_API_KEY=your_etherscan_api_key_here
```

## Contract Features

- **ERC20 Token**: DEMLE tokens rewarded for ML mining
- **Mining Proof Submission**: Verify FP8 ML computations
- **Difficulty Adjustment**: Dynamic mining difficulty
- **Max Supply**: 21 million tokens total
- **Mining Reward**: 100 DEMLE per valid proof

## Note

The Node.js version warning is cosmetic - Hardhat works fine with Node.js v23, though v18 LTS is officially recommended. 