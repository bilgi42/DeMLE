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

- `demle-dashboard.html` - Interactive web dashboard
- Console output - Real-time mining simulation
- Contract addresses - For integration testing

## 🎮 Try It Now!

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

After running the demos, you'll see:
- **300-1400 DEMLE tokens** distributed among miners
- **70-100% success rate** for mining operations
- **Individual miner holdings** ranging from 0-400 DEMLE
- **Real-time distribution statistics** and analytics

This demonstrates how your FP8 ML computations are rewarded with DEMLE tokens in a decentralized manner!

## 🚀 Next Steps

1. **Connect Your Rust Miner**: Use the contract address from demos
2. **Submit Real ML Proofs**: Replace demo data with actual FP8 computations
3. **Monitor Token Distribution**: Use the dashboard to track real mining progress
4. **Scale Up**: Deploy to testnet for multi-user mining 