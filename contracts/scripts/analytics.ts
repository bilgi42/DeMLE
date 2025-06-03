import { ethers } from "hardhat";

async function main() {
  console.log("📊 DEMLE Token Analytics Dashboard");
  console.log("=".repeat(50));

  // Deploy the contract for demo
  console.log("\n🚀 Deploying DEMLE contract...");
  const DEMLE = await ethers.getContractFactory("DEMLE");
  const demle = await DEMLE.deploy();
  await demle.waitForDeployment();
  
  const contractAddress = await demle.getAddress();
  console.log(`✅ Contract deployed at: ${contractAddress}`);

  // Get some accounts to simulate miners
  const accounts = await ethers.getSigners();
  const owner = accounts[0];
  const miners = accounts.slice(1, 4); // Use 3 miners

  console.log("\n💼 Contract Information:");
  console.log(`🏷️  Name: ${await demle.name()}`);
  console.log(`🎯 Symbol: ${await demle.symbol()}`);
  console.log(`💰 Mining Reward: ${ethers.formatEther(await demle.MINING_REWARD())} DEMLE`);
  console.log(`🎪 Max Supply: ${ethers.formatEther(await demle.MAX_SUPPLY())} DEMLE`);
  console.log(`👑 Owner: ${await demle.owner()}`);

  // Simulate some mining activity
  console.log("\n⛏️  Simulating Mining Activity...");
  
  for (let i = 0; i < miners.length; i++) {
    const miner = miners[i];
    const nonce = ethers.randomBytes(32);
    
    // Create realistic ML proof data
    const mlProof = JSON.stringify({
      operation: "gemm",
      input_shape: [128, 256],
      output_shape: [128, 512],
      fp8_precision: "E4M3",
      result_hash: ethers.keccak256(ethers.randomBytes(32))
    });
    
    try {
      console.log(`  👤 Miner ${i + 1} (${miner.address.slice(0, 8)}...) mining...`);
      const tx = await demle.connect(miner).submitMiningProof(nonce, ethers.toUtf8Bytes(mlProof));
      await tx.wait();
      console.log(`     ✅ Mining successful!`);
    } catch (error: any) {
      console.log(`     ❌ Mining failed: ${error.message.split('(')[0]}`);
    }
  }

  // Show token distribution
  console.log("\n📈 Current Token Distribution:");
  console.log("-".repeat(40));
  
  const totalSupply = await demle.totalSupply();
  console.log(`🏦 Total Supply: ${ethers.formatEther(totalSupply)} DEMLE`);
  
  let totalDistributed = BigInt(0);
  const minerBalances = [];
  
  for (let i = 0; i < miners.length; i++) {
    const balance = await demle.balanceOf(miners[i].address);
    const lastMining = await demle.lastMiningTime(miners[i].address);
    const formattedBalance = ethers.formatEther(balance);
    
    minerBalances.push({
      index: i + 1,
      address: miners[i].address,
      balance: formattedBalance,
      lastMining: lastMining.toString(),
      hasMined: lastMining > 0
    });
    
    totalDistributed += balance;
    
    console.log(`👤 Miner ${i + 1}: ${formattedBalance} DEMLE ${lastMining > 0 ? '✅' : '⏳'}`);
    console.log(`   Address: ${miners[i].address}`);
    console.log(`   Last Mining: ${lastMining > 0 ? new Date(Number(lastMining) * 1000).toISOString() : 'Never'}`);
  }

  // Analytics summary
  console.log("\n📊 Analytics Summary:");
  console.log("-".repeat(40));
  console.log(`💼 Total Distributed: ${ethers.formatEther(totalDistributed)} DEMLE`);
  console.log(`📈 Distribution Progress: ${(Number(totalDistributed) / Number(await demle.MAX_SUPPLY()) * 100).toFixed(8)}%`);
  console.log(`⚡ Current Difficulty: ${await demle.getMiningDifficulty()}`);
  console.log(`🎯 Successful Miners: ${minerBalances.filter(m => m.hasMined).length}/${miners.length}`);

  // Show mining statistics
  const activeMinerCount = minerBalances.filter(m => m.hasMined).length;
  const avgTokensPerMiner = activeMinerCount > 0 ? Number(totalDistributed) / activeMinerCount : 0;
  
  console.log(`📊 Average tokens per active miner: ${ethers.formatEther(avgTokensPerMiner)} DEMLE`);
  
  // Show top miner
  const topMiner = minerBalances.reduce((prev, current) => 
    parseFloat(prev.balance) > parseFloat(current.balance) ? prev : current
  );
  
  if (topMiner.hasMined) {
    console.log(`🏆 Top Miner: Miner ${topMiner.index} with ${topMiner.balance} DEMLE`);
  }

  console.log("\n🔗 Contract Details for Integration:");
  console.log("-".repeat(40));
  console.log(`📍 Contract Address: ${contractAddress}`);
  console.log(`🔧 ABI Functions: submitMiningProof, getMiningDifficulty, balanceOf`);
  console.log(`📡 Events: MiningReward(miner, amount, nonce)`);
  
  console.log("\n✨ Demo completed! Use this contract address to interact with your Rust miner.");
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  }); 