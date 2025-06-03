import { ethers } from "hardhat";

async function main() {
  console.log("🚀 DEMLE Token Distribution Demo");
  console.log("=".repeat(50));

  // Get accounts
  const [owner, miner1, miner2, miner3] = await ethers.getSigners();
  
  // Deploy contract
  console.log("\n📋 Deploying DEMLE contract...");
  const DEMLE = await ethers.getContractFactory("DEMLE");
  const demle = await DEMLE.deploy();
  await demle.waitForDeployment();
  
  const contractAddress = await demle.getAddress();
  console.log(`✅ DEMLE deployed to: ${contractAddress}`);

  // Show initial state
  console.log("\n💼 Contract Information:");
  console.log(`🏷️  Name: ${await demle.name()}`);
  console.log(`🎯 Symbol: ${await demle.symbol()}`);
  console.log(`💰 Mining Reward: ${ethers.formatEther(await demle.MINING_REWARD())} DEMLE`);
  console.log(`🎪 Max Supply: ${ethers.formatEther(await demle.MAX_SUPPLY())} DEMLE`);

  const miners = [
    { signer: miner1, name: "Miner 1" },
    { signer: miner2, name: "Miner 2" }, 
    { signer: miner3, name: "Miner 3" }
  ];

  // Function to show current balances
  async function showBalances() {
    console.log("\n📊 Current Token Distribution:");
    console.log("-".repeat(40));
    
    const totalSupply = await demle.totalSupply();
    console.log(`🏦 Total Supply: ${ethers.formatEther(totalSupply)} DEMLE`);
    
    for (const miner of miners) {
      const balance = await demle.balanceOf(miner.signer.address);
      const lastMining = await demle.lastMiningTime(miner.signer.address);
      console.log(`👤 ${miner.name}: ${ethers.formatEther(balance)} DEMLE ${lastMining > 0 ? '✅' : '⏳'}`);
      console.log(`   Address: ${miner.signer.address}`);
    }
  }

  // Show initial balances (should be 0)
  await showBalances();

  // Simulate mining for each miner
  console.log("\n⛏️  Simulating Mining Operations...");
  
  for (let i = 0; i < miners.length; i++) {
    const miner = miners[i];
    const nonce = ethers.randomBytes(32);
    
    // Create ML proof data that represents FP8 operations
    const mlProofData = {
      operation_type: "gemm", // Matrix multiplication
      input_dimensions: [128, 256],
      output_dimensions: [128, 512], 
      precision: "FP8_E4M3",
      computation_result: ethers.hexlify(ethers.randomBytes(32)),
      timestamp: Math.floor(Date.now() / 1000)
    };
    
    const mlProof = ethers.toUtf8Bytes(JSON.stringify(mlProofData));
    
    try {
      console.log(`\n${miner.name} attempting to mine...`);
      console.log(`  📊 Proof size: ${mlProof.length} bytes`);
      console.log(`  🔢 Nonce: ${ethers.hexlify(nonce).slice(0, 10)}...`);
      
      const tx = await demle.connect(miner.signer).submitMiningProof(nonce, mlProof);
      const receipt = await tx.wait();
      
      console.log(`  ✅ Mining successful!`);
      console.log(`  ⛽ Gas used: ${receipt?.gasUsed.toString()}`);
      console.log(`  💰 Earned: ${ethers.formatEther(await demle.MINING_REWARD())} DEMLE`);
      
    } catch (error: any) {
      console.log(`  ❌ Mining failed: ${error.message.split('(')[0]}`);
    }
  }

  // Show final distribution
  await showBalances();

  // Analytics
  console.log("\n📈 Mining Analytics:");
  console.log("-".repeat(40));
  
  let totalEarned = BigInt(0);
  let successfulMiners = 0;
  
  for (const miner of miners) {
    const balance = await demle.balanceOf(miner.signer.address);
    const lastMining = await demle.lastMiningTime(miner.signer.address);
    
    if (balance > 0) {
      totalEarned += balance;
      successfulMiners++;
    }
  }
  
  const difficulty = await demle.getMiningDifficulty();
  const maxSupply = await demle.MAX_SUPPLY();
  const distributionPercentage = (Number(totalEarned) / Number(maxSupply)) * 100;
  
  console.log(`💼 Total DEMLE Distributed: ${ethers.formatEther(totalEarned)} DEMLE`);
  console.log(`🎯 Successful Miners: ${successfulMiners}/${miners.length}`);
  console.log(`⚡ Current Difficulty: ${difficulty}`);
  console.log(`📊 Distribution Progress: ${distributionPercentage.toFixed(8)}%`);
  
  if (successfulMiners > 0) {
    const avgEarnings = Number(totalEarned) / successfulMiners;
    console.log(`📊 Average Earnings: ${ethers.formatEther(avgEarnings)} DEMLE per successful miner`);
  }

  // Show how to interact with the contract
  console.log("\n🔗 Integration Information:");
  console.log("-".repeat(40));
  console.log(`📍 Contract Address: ${contractAddress}`);
  console.log(`🎯 Network: Hardhat Local (Chain ID: 31337)`);
  console.log(`\n🛠️  Key Functions:`);
  console.log(`  • submitMiningProof(nonce, mlProof) - Submit FP8 ML computation proof`);
  console.log(`  • balanceOf(address) - Check DEMLE token balance`);
  console.log(`  • getMiningDifficulty() - Get current mining difficulty`);
  console.log(`  • totalSupply() - Get total DEMLE tokens minted`);
  
  console.log(`\n📡 Events to Monitor:`);
  console.log(`  • MiningReward(miner, amount, nonce) - Successful mining event`);
  
  console.log("\n✨ Demo Complete! Your Rust miner can now interact with this contract.");
  console.log(`   Use contract address: ${contractAddress}`);
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  }); 