import { ethers } from "hardhat";

async function main() {
  console.log("💰 DEMLE Token Balance Checker");
  console.log("=".repeat(50));

  // Connect to your deployed contract
  const contractAddress = "0x5FbDB2315678afecb367f032d93F642f64180aa3";
  console.log(`📍 Contract Address: ${contractAddress}`);
  
  // Get the contract instance
  const DEMLE = await ethers.getContractFactory("DEMLE");
  const demle = DEMLE.attach(contractAddress);

  // Get all accounts to check their balances
  const accounts = await ethers.getSigners();
  
  try {
    // Contract stats
    const totalSupply = await demle.totalSupply();
    const maxSupply = await demle.MAX_SUPPLY();
    const miningReward = await demle.MINING_REWARD();
    const difficulty = await demle.getMiningDifficulty();

    console.log("\n🏦 Contract Statistics:");
    console.log(`   Total Minted: ${ethers.formatEther(totalSupply)} DEMLE`);
    console.log(`   Max Supply: ${ethers.formatEther(maxSupply)} DEMLE`);
    console.log(`   Mining Reward: ${ethers.formatEther(miningReward)} DEMLE`);
    console.log(`   Current Difficulty: ${difficulty}`);
    
    const distributionPercent = (Number(totalSupply) / Number(maxSupply)) * 100;
    console.log(`   Distribution: ${distributionPercent.toFixed(8)}%`);

    console.log("\n👥 Account Balances:");
    let totalHoldings = BigInt(0);
    const activeMiners: any[] = [];

    // Check balances for all accounts
    for (let i = 0; i < Math.min(accounts.length, 10); i++) {
      const account = accounts[i];
      const balance = await demle.balanceOf(account.address);
      const lastMiningTime = await demle.lastMiningTime(account.address);
      
      if (balance > 0 || lastMiningTime > 0) {
        const balanceFormatted = ethers.formatEther(balance);
        totalHoldings += balance;
        
        const status = lastMiningTime > 0 ? "✅ Active Miner" : "⏳ Not Mined";
        console.log(`   Account ${i}: ${balanceFormatted} DEMLE ${status}`);
        console.log(`      Address: ${account.address}`);
        
        if (lastMiningTime > 0) {
          const lastMineDate = new Date(Number(lastMiningTime) * 1000);
          console.log(`      Last Mining: ${lastMineDate.toISOString()}`);
          activeMiners.push({
            address: account.address,
            balance: balance,
            lastMiningTime: lastMiningTime
          });
        }
        console.log("");
      }
    }

    // Summary
    console.log("📈 Mining Summary:");
    console.log(`   Active Miners: ${activeMiners.length}`);
    console.log(`   Total Token Holdings: ${ethers.formatEther(totalHoldings)} DEMLE`);
    
    if (activeMiners.length > 0) {
      const avgBalance = Number(totalHoldings) / activeMiners.length;
      console.log(`   Average per Miner: ${ethers.formatEther(BigInt(Math.floor(avgBalance)))} DEMLE`);
      
      // Show top miners
      activeMiners.sort((a, b) => Number(b.balance) - Number(a.balance));
      console.log("\n🏆 Top Miners:");
      activeMiners.slice(0, 5).forEach((miner, index) => {
        console.log(`   ${index + 1}. ${miner.address}: ${ethers.formatEther(miner.balance)} DEMLE`);
      });
    }

    console.log("\n💡 How to use this contract:");
    console.log("   - Each successful ML proof submission earns 100 DEMLE tokens");
    console.log("   - Miners must provide unique nonces to prevent double-spending");
    console.log("   - ML proof must contain valid FP8 computation data");
    
  } catch (error) {
    console.error("❌ Error reading contract:", error);
    console.log("\n💡 Make sure the contract is deployed and accessible");
    console.log("   If this is a fresh deployment, mining transactions need to be submitted first");
  }
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  }); 