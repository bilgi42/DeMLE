# 🚀 DEMLE Token Distribution Demo Guide

This guide shows you multiple ways to visualize and track DEMLE token distribution from your ML mining operations.

## 🎯 Quick Demo Commands

```bash
# 1. Simple interactive demo - shows mining simulation
npm run demo

# 2. Check token balances and statistics 
npm run check-balances

# 3. Generate visual web dashboard
npm run dashboard

# 4. 🆕 REAL-TIME Dashboard - Updates live as miners work!
npm run realtime-demo
```

## 📊 What You'll See

### 1. **Interactive Mining Demo** (`npm run demo`)
- ✅ Simulates 3 miners performing FP8 ML computations
- 📈 Shows real-time token distribution as mining happens
- 🏆 Displays final leaderboard of token holders
- ⛽ Shows gas costs and mining statistics

**Sample Output:**
```
🚀 DEMLE Token Distribution Demo
==================================================

👥 Setting up 3 miners...
✅ DEMLE deployed to: 0x5FbDB2315678afecb367f032d93F642f64180aa3
💰 Mining Reward: 100.0 DEMLE

⛏️ Simulating Mining Operations...
Miner 1 attempting to mine...
  ✅ Mining successful!
  💰 Earned: 100.0 DEMLE

📊 Current Token Distribution:
👤 Miner 1: 100.0 DEMLE ✅
👤 Miner 2: 100.0 DEMLE ✅  
👤 Miner 3: 100.0 DEMLE ✅
```

### 2. **Balance Checker** (`npm run check-balances`)
- 💰 Shows detailed balance information for each miner
- 📈 Displays mining statistics and success rates
- 🎯 Provides contract integration instructions
- ⏰ Shows last mining timestamps

**Key Features:**
- Individual miner balances
- Mining success rates
- Total token distribution percentage
- Contract address for integration

### 3. **Web Dashboard** (`npm run dashboard`)
- 🌐 Generates a beautiful HTML dashboard (`demle-dashboard.html`)
- 📊 Interactive charts showing token distribution
- 👥 Miner profiles with avatars and statistics
- 📝 Mining activity log
- 🔗 Contract integration information

**Dashboard Features:**
- **Visual Charts**: Pie chart showing token distribution
- **Miner Cards**: Individual profiles with holdings and success rates
- **Statistics**: Total supply, distribution progress, mining difficulty
- **Activity Log**: Real-time mining operations history
- **Contract Info**: Address and integration details

### 4. **🆕 Real-time Dashboard** (`npm run realtime-demo`) - **RECOMMENDED!**
- 🔥 **Connects to actual running miners**
- ⚡ **Updates every 3 seconds automatically**
- 🎯 **Shows YOUR miner as it earns tokens**
- 📱 **Works with 1 miner or multiple miners**
- 🎨 **Beautiful animations when mining succeeds**
- 📊 **Live charts and statistics**

**Key Features:**
- **Live Monitoring**: Detects when your Rust miner connects and mines
- **Real-time Updates**: Shows token balances updating as you earn DEMLE
- **Multi-Miner Support**: If you run 2+ miners, shows all of them
- **Mining Events**: Live feed of successful mining operations
- **Visual Highlights**: Miner cards glow when they successfully mine
- **Click to Copy**: Contract address copies to clipboard

## 🔧 How DEMLE Token Distribution Works

### Mining Process
1. **ML Computation**: Miners perform FP8 operations (GEMM, Conv2D, Attention, BatchNorm)
2. **Proof Submission**: Submit computation proof with unique nonce
3. **Verification**: Smart contract verifies the ML proof
4. **Token Reward**: Successful miners receive 100 DEMLE tokens
5. **Nonce Prevention**: Each nonce can only be used once

### Token Economics
- **Max Supply**: 21,000,000 DEMLE tokens
- **Mining Reward**: 100 DEMLE per successful proof
- **Difficulty**: Adjusts based on total supply
- **Standard**: ERC-20 compatible

## 🆕 Real-time Dashboard Setup

### Quick Start (RECOMMENDED):
```bash
# Terminal 1: Start the real-time dashboard
npm run realtime-demo

# Terminal 2: Start Hardhat node (if not running)
npx hardhat node

# Terminal 3: Run your miner
cargo run --bin demle-miner -- --contract YOUR_CONTRACT_ADDRESS --rpc http://localhost:8545

# Open realtime-dashboard.html in your browser
# Watch your miner appear and earn tokens live!
```

### What You'll See in Real-time:
- **Your miner appears** as soon as it submits its first proof
- **Token balance updates** every 3 seconds 
- **Mining events** show up immediately with animations
- **Charts update** automatically as distribution changes
- **Multiple miners** are supported if you run more than one

## 🧪 Demo Data Examples

The demos show realistic ML proof data:
```json
{
  "operation_type": "gemm",
  "input_dimensions": [128, 256],
  "output_dimensions": [128, 512],
  "precision": "FP8_E4M3",
  "computation_result": "0x...",
  "timestamp": 1639123456
}
```

## 🔗 Integration with Your Rust Miner

Use the contract address from any demo to connect your Rust miner:

```rust
// Example integration
let contract_address = "0x5FbDB2315678afecb367f032d93F642f64180aa3";
let balance = contract.balance_of(miner_address).call().await?;
let tx = contract.submit_mining_proof(nonce, ml_proof).send().await?;
```

## 📁 Generated Files

- `realtime-dashboard.html` - 🆕 **Live dashboard that updates automatically**
- `demle-dashboard.html` - Static interactive web dashboard
- Console output - Real-time mining simulation
- Contract addresses - For integration testing

## 🎮 Try It Now!

### For Real Mining (RECOMMENDED):
```bash
# Start the live dashboard
npm run realtime-demo

# In another terminal, start your miner
# The dashboard will show your miner and update live!
```

### For Quick Demos:
1. **Start with the simple demo:**
   ```bash
   npm run demo
   ```

2. **Check detailed balances:**
   ```bash
   npm run check-balances
   ```

3. **Generate visual dashboard:**
   ```bash
   npm run dashboard
   # Then open demle-dashboard.html in your browser
   ```

## 🏆 Expected Results

### With Real-time Dashboard:
- **Your actual miner appears** when you start it
- **Token balance grows** as you submit successful ML proofs
- **Live events** show each mining success with timestamps
- **Visual feedback** when mining operations complete
- **Works with multiple miners** if you run them simultaneously

### With Static Demos:
- **300-1400 DEMLE tokens** distributed among simulated miners
- **70-100% success rate** for mining operations
- **Individual miner holdings** ranging from 0-400 DEMLE
- **Real-time distribution statistics** and analytics

This demonstrates how your FP8 ML computations are rewarded with DEMLE tokens in a decentralized manner!

## 🚀 Next Steps

1. **🔥 Use Real-time Dashboard**: `npm run realtime-demo` for live monitoring
2. **Connect Your Rust Miner**: Use the contract address from dashboard
3. **Submit Real ML Proofs**: Replace demo data with actual FP8 computations
4. **Monitor Live Progress**: Watch tokens accumulate in real-time
5. **Scale Up**: Deploy to testnet for multi-user mining

**The real-time dashboard is perfect for development and testing - you can see exactly when your miner succeeds and how tokens are distributed!** 