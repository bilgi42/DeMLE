import { ethers } from "hardhat";

async function main() {
  console.log("🔍 DEMLE Token Balance Checker - Live Contracts");
  console.log("=".repeat(60));

  // Get the most recent blocks to find deployed contracts
  const provider = ethers.provider;
  const currentBlock = await provider.getBlockNumber();
  const searchBlocks = Math.min(100, currentBlock); // Search last 100 blocks max
  
  console.log(`📊 Scanning last ${searchBlocks} blocks for DEMLE deployments...`);
  
  const deployedContracts = [];
  
  // Search for contract deployments in recent blocks
  for (let i = Math.max(1, currentBlock - searchBlocks); i <= currentBlock; i++) {
    try {
      const block = await provider.getBlock(i, true);
      if (block && block.transactions) {
        for (const tx of block.transactions) {
          if (typeof tx === 'object' && tx !== null && 'to' in tx && 'data' in tx && 'hash' in tx) {
            const transaction = tx as any; // Type assertion for transaction object
            if (transaction.to === null && transaction.data) {
              // This is a contract deployment
              const receipt = await provider.getTransactionReceipt(transaction.hash);
              if (receipt && receipt.contractAddress) {
                // Try to verify if it's a DEMLE contract
                try {
                  const code = await provider.getCode(receipt.contractAddress);
                  if (code && code !== '0x') {
                    const DEMLE = await ethers.getContractFactory("DEMLE");
                    const testContract = DEMLE.attach(receipt.contractAddress);
                    
                    // Test if it has DEMLE methods
                    await testContract.MINING_REWARD();
                    deployedContracts.push(receipt.contractAddress);
                    console.log(`✅ Found DEMLE contract: ${receipt.contractAddress} (Block ${i})`);
                  }
                } catch (e) {
                  // Not a DEMLE contract or error reading it
                }
              }
            }
          }
        }
      }
    } catch (e) {
      // Skip blocks that can't be read
    }
  }

  if (deployedContracts.length === 0) {
    console.log("❌ No DEMLE contracts found in recent blocks!");
    console.log("\n💡 To deploy a new contract:");
    console.log("   npx hardhat run scripts/deploy.ts --network localhost");
    console.log("\n📋 Or run the complete setup:");
    console.log("   npm run setup-demo");
    return;
  }

  const DEMLE = await ethers.getContractFactory("DEMLE");
  const accounts = await ethers.getSigners();

  for (let i = 0; i < deployedContracts.length; i++) {
    const contractAddress = deployedContracts[i];
    console.log(`\n📍 Contract #${i + 1}: ${contractAddress}`);
    console.log("-".repeat(50));

    try {
      const demle = DEMLE.attach(contractAddress);

      // Contract stats
      const totalSupply = await demle.totalSupply();
      const maxSupply = await demle.MAX_SUPPLY();
      const miningReward = await demle.MINING_REWARD();
      const difficulty = await demle.getMiningDifficulty();

      console.log(`🏦 Total Minted: ${ethers.formatEther(totalSupply)} DEMLE`);
      console.log(`💰 Mining Reward: ${ethers.formatEther(miningReward)} DEMLE`);
      console.log(`⚡ Difficulty: ${difficulty}`);
      
      const distributionPercent = (Number(totalSupply) / Number(maxSupply)) * 100;
      console.log(`📊 Distribution: ${distributionPercent.toFixed(8)}%`);

      // Check balances for all accounts
      console.log("\n👥 Account Balances:");
      let totalHoldings = BigInt(0);
      const miners: any[] = [];

      for (let j = 0; j < Math.min(accounts.length, 10); j++) {
        const account = accounts[j];
        const balance = await demle.balanceOf(account.address);
        const lastMiningTime = await demle.lastMiningTime(account.address);
        
        if (balance > 0 || lastMiningTime > 0) {
          const balanceFormatted = ethers.formatEther(balance);
          totalHoldings += balance;
          
          const status = lastMiningTime > 0 ? "✅ Active Miner" : "⏳ No Mining";
          console.log(`   Account ${j}: ${balanceFormatted} DEMLE ${status}`);
          console.log(`      Address: ${account.address.slice(0, 10)}...${account.address.slice(-8)}`);
          
          if (lastMiningTime > 0) {
            const lastMineDate = new Date(Number(lastMiningTime) * 1000);
            console.log(`      Last Mining: ${lastMineDate.toLocaleString()}`);
            miners.push({
              address: account.address,
              balance: balance,
              lastMiningTime: lastMiningTime
            });
          }
        }
      }

      if (miners.length > 0) {
        console.log(`\n📈 Summary for Contract #${i + 1}:`);
        console.log(`   Active Miners: ${miners.length}`);
        console.log(`   Total Holdings: ${ethers.formatEther(totalHoldings)} DEMLE`);
        
        const avgBalance = Number(totalHoldings) / miners.length;
        console.log(`   Avg per Miner: ${ethers.formatEther(BigInt(Math.floor(avgBalance)))} DEMLE`);
        
        // Show recent mining activity
        miners.sort((a, b) => Number(b.lastMiningTime) - Number(a.lastMiningTime));
        console.log("\n🏆 Most Recent Miners:");
        miners.slice(0, 3).forEach((miner, index) => {
          const recent = new Date(Number(miner.lastMiningTime) * 1000);
          console.log(`   ${index + 1}. ${miner.address.slice(0, 10)}...${miner.address.slice(-8)}: ${ethers.formatEther(miner.balance)} DEMLE (${recent.toLocaleString()})`);
        });
      } else {
        console.log("   No active miners found on this contract");
      }

    } catch (error) {
      console.error(`❌ Error reading contract #${i + 1}:`, (error as Error).message);
    }
  }

  console.log("\n" + "=".repeat(60));
  if (deployedContracts.length > 0) {
    console.log("💡 Quick Start:");
    const latestContract = deployedContracts[deployedContracts.length - 1];
    console.log(`   Use latest contract: ${latestContract}`);
    console.log(`   Mining command: cargo run --bin demle-miner -- --contract ${latestContract} --rpc-url http://localhost:8545`);
  }
  console.log("\n🔧 Need a fresh start?");
  console.log("   1. Run: npm run setup-demo");
  console.log("   2. This will deploy a new contract and create a dashboard");
  console.log("   3. Follow the instructions to start mining!");
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  }); 