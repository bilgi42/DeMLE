import { ethers } from "hardhat";

async function main() {
  // This script assumes you have a deployed contract address
  // If not, it will deploy a new one for demo
  
  console.log("💰 DEMLE Token Balance Checker");
  console.log("=".repeat(50));

  // Get accounts
  const [owner, miner1, miner2, miner3] = await ethers.getSigners();
  
  // Deploy contract for demo
  console.log("\n🚀 Deploying new DEMLE contract for demo...");
  const DEMLE = await ethers.getContractFactory("DEMLE");
  const demle = await DEMLE.deploy();
  await demle.waitForDeployment();
  const contractAddress = await demle.getAddress();
  
  console.log(`📍 Contract Address: ${contractAddress}`);

  // Quick mining demo
  console.log("\n⛏️  Quick Mining Demo...");
  
  const miners = [
    { signer: miner1, name: "Alice" },
    { signer: miner2, name: "Bob" },
    { signer: miner3, name: "Charlie" }
  ];

  // Simulate mining for demonstration
  for (let i = 0; i < 2; i++) { // Mine twice for variety
    for (const miner of miners) {
      const nonce = ethers.randomBytes(32);
      const mlProof = ethers.toUtf8Bytes(JSON.stringify({
        gemm_operation: "matrix_multiply",
        fp8_precision: "E4M3",
        result: ethers.hexlify(ethers.randomBytes(16))
      }));
      
      try {
        await demle.connect(miner.signer).submitMiningProof(nonce, mlProof);
        console.log(`  ✅ ${miner.name} mined successfully!`);
      } catch (error) {
        console.log(`  ❌ ${miner.name} mining failed (probably duplicate nonce)`);
      }
    }
  }

  // Now show the token distribution
  console.log("\n📊 CURRENT TOKEN DISTRIBUTION");
  console.log("=".repeat(50));

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

  // Individual balances
  console.log("\n👥 Individual Balances:");
  let totalHoldings = BigInt(0);
  const balances = [];

  for (const miner of miners) {
    const balance = await demle.balanceOf(miner.signer.address);
    const lastMiningTime = await demle.lastMiningTime(miner.signer.address);
    const balanceFormatted = ethers.formatEther(balance);
    
    balances.push({
      name: miner.name,
      address: miner.signer.address,
      balance: balance,
      formatted: balanceFormatted,
      hasMined: lastMiningTime > 0
    });
    
    totalHoldings += balance;
    
    const status = lastMiningTime > 0 ? "✅ Active Miner" : "⏳ Not Mined";
    console.log(`   ${miner.name}: ${balanceFormatted} DEMLE ${status}`);
    console.log(`      Address: ${miner.signer.address}`);
    
    if (lastMiningTime > 0) {
      const lastMineDate = new Date(Number(lastMiningTime) * 1000);
      console.log(`      Last Mining: ${lastMineDate.toISOString()}`);
    }
  }

  // Summary
  console.log("\n📈 Distribution Summary:");
  const activeMiners = balances.filter(b => b.hasMined).length;
  console.log(`   Active Miners: ${activeMiners}/${miners.length}`);
  console.log(`   Total Holdings: ${ethers.formatEther(totalHoldings)} DEMLE`);
  
  if (activeMiners > 0) {
    const avgBalance = Number(totalHoldings) / activeMiners;
    console.log(`   Average per Active Miner: ${ethers.formatEther(BigInt(Math.floor(avgBalance)))} DEMLE`);
  }

  // Top holder
  const topHolder = balances.reduce((prev, current) => 
    prev.balance > current.balance ? prev : current
  );
  
  if (topHolder.balance > 0) {
    console.log(`   🏆 Top Holder: ${topHolder.name} with ${topHolder.formatted} DEMLE`);
  }

  // Instructions for integration
  console.log("\n🔗 Integration Instructions:");
  console.log("=".repeat(50));
  console.log(`📍 Contract Address: ${contractAddress}`);
  console.log(`🌐 Network: Hardhat Local (chainId: 31337)`);
  console.log("\n📝 To check balances in your Rust code:");
  console.log('   let balance = contract.balance_of(miner_address).call().await?;');
  console.log("\n📝 To submit mining proof:");
  console.log('   let tx = contract.submit_mining_proof(nonce, ml_proof).send().await?;');
  
  console.log("\n✨ Demo completed! This shows how DEMLE tokens are distributed to miners.");
  console.log("   Each successful ML computation proof earns 100 DEMLE tokens.");
  console.log("   The contract prevents double-spending with unique nonces.");
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  }); 