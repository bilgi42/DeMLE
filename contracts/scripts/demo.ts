import { ethers } from "hardhat";
import { DEMLE } from "../typechain-types";
import { HardhatEthersSigner } from "@nomicfoundation/hardhat-ethers/signers";

async function main() {
  console.log("🚀 DEMLE Token Distribution Demo");
  console.log("=".repeat(50));

  // Get signers (simulate different miners)
  const signers = await ethers.getSigners();
  const deployer = signers[0];
  const miners = signers.slice(1, 6); // Use 5 different miners

  console.log(`\n👥 Setting up ${miners.length} miners...`);
  
  // Deploy contract
  console.log("\n📋 Deploying DEMLE contract...");
  const DEMLE = await ethers.getContractFactory("DEMLE");
  const demleDeployment = await DEMLE.deploy();
  await demleDeployment.waitForDeployment();
  const contractAddress = await demleDeployment.getAddress();
  
  // Get properly typed contract instance
  const demle = DEMLE.attach(contractAddress) as DEMLE;
  
  console.log(`✅ DEMLE deployed to: ${contractAddress}`);
  console.log(`💰 Mining reward: ${ethers.formatEther(await demle.MINING_REWARD())} DEMLE`);
  console.log(`🎯 Max supply: ${ethers.formatEther(await demle.MAX_SUPPLY())} DEMLE`);

  // Function to display current token distribution
  async function showDistribution() {
    console.log("\n" + "=".repeat(60));
    console.log("📊 CURRENT TOKEN DISTRIBUTION");
    console.log("=".repeat(60));
    
    const totalSupply = await demle.totalSupply();
    console.log(`🏦 Total Supply: ${ethers.formatEther(totalSupply)} DEMLE`);
    console.log(`📈 Difficulty: ${await demle.getMiningDifficulty()}`);
    
    console.log("\n👥 Miner Balances:");
    let totalDistributed = 0n;
    
    for (let i = 0; i < miners.length; i++) {
      const balance = await demle.balanceOf(miners[i].address);
      const lastMining = await demle.lastMiningTime(miners[i].address);
      const formattedBalance = ethers.formatEther(balance);
      
      console.log(`  Miner ${i + 1} (${miners[i].address.slice(0, 8)}...): ${formattedBalance} DEMLE ${lastMining > 0 ? '✅' : '⏳'}`);
      totalDistributed += balance;
    }
    
    console.log(`\n💼 Total Distributed: ${ethers.formatEther(totalDistributed)} DEMLE`);
    console.log(`📊 Distribution Progress: ${(Number(totalDistributed) / Number(await demle.MAX_SUPPLY()) * 100).toFixed(4)}%`);
  }

  // Function to simulate mining with realistic ML proof data
  async function simulateMining(miner: HardhatEthersSigner, minerIndex: number): Promise<boolean> {
    try {
      // Generate realistic looking ML proof data
      const nonce = ethers.randomBytes(32);
      const mlOperations = {
        gemm_result: Array.from({length: 16}, () => Math.random()),
        conv2d_result: Array.from({length: 8}, () => Math.random()), 
        attention_weights: Array.from({length: 4}, () => Math.random()),
        batch_norm_stats: { mean: Math.random(), variance: Math.random() }
      };
      
      const mlProof = ethers.toUtf8Bytes(JSON.stringify(mlOperations));
      
      console.log(`\n⛏️  Miner ${minerIndex + 1} submitting ML proof...`);
      console.log(`   📊 Proof size: ${mlProof.length} bytes`);
      console.log(`   🔢 Nonce: ${nonce.slice(0, 10)}...`);
      
      const tx = await demle.connect(miner).submitMiningProof(nonce, mlProof);
      const receipt = await tx.wait();
      
      // Find the MiningReward event
      const event = receipt?.logs.find((log: any) => {
        try {
          const parsed = demle.interface.parseLog(log);
          return parsed?.name === 'MiningReward';
        } catch {
          return false;
        }
      });
      
      if (event) {
        const parsed = demle.interface.parseLog(event);
        console.log(`   ✅ Mining successful! Earned ${ethers.formatEther(parsed?.args.amount)} DEMLE`);
        console.log(`   ⛽ Gas used: ${receipt?.gasUsed.toString()}`);
      }
      
      return true;
    } catch (error: any) {
      console.log(`   ❌ Mining failed: ${error.message.split('(')[0]}`);
      return false;
    }
  }

  // Show initial state
  await showDistribution();

  // Simulate mining rounds
  console.log("\n\n🎮 Starting Mining Simulation...");
  console.log("Each round, random miners will attempt to mine...\n");

  for (let round = 1; round <= 10; round++) {
    console.log(`\n🔄 ROUND ${round}`);
    console.log("-".repeat(30));
    
    // Randomly select 1-3 miners to attempt mining this round
    const numMiners = Math.floor(Math.random() * 3) + 1;
    const selectedMiners: number[] = [];
    
    for (let i = 0; i < numMiners; i++) {
      const randomIndex = Math.floor(Math.random() * miners.length);
      if (!selectedMiners.includes(randomIndex)) {
        selectedMiners.push(randomIndex);
      }
    }
    
    // Each selected miner attempts to mine
    for (const minerIndex of selectedMiners) {
      await simulateMining(miners[minerIndex], minerIndex);
      
      // Small delay for readability
      await new Promise(resolve => setTimeout(resolve, 500));
    }
    
    // Show updated distribution
    await showDistribution();
    
    // Pause between rounds
    if (round < 10) {
      console.log("\n⏳ Waiting for next round...");
      await new Promise(resolve => setTimeout(resolve, 1000));
    }
  }

  // Final summary
  console.log("\n\n🎉 DEMO COMPLETE!");
  console.log("=".repeat(50));
  console.log("📋 Summary:");
  
  const finalSupply = await demle.totalSupply();
  const maxSupply = await demle.MAX_SUPPLY();
  const percentMined = (Number(finalSupply) / Number(maxSupply) * 100);
  
  console.log(`💰 Total tokens mined: ${ethers.formatEther(finalSupply)} DEMLE`);
  console.log(`📊 Percentage of max supply: ${percentMined.toFixed(6)}%`);
  console.log(`🎯 Contract address: ${contractAddress}`);
  
  // Show final distribution by miner
  console.log("\n🏆 Final Leaderboard:");
  const balances = [];
  for (let i = 0; i < miners.length; i++) {
    const balance = await demle.balanceOf(miners[i].address);
    balances.push({ 
      index: i + 1, 
      address: miners[i].address, 
      balance: balance,
      formatted: ethers.formatEther(balance)
    });
  }
  
  balances
    .sort((a, b) => Number(b.balance - a.balance))
    .forEach((miner, rank) => {
      const medal = rank === 0 ? "🥇" : rank === 1 ? "🥈" : rank === 2 ? "🥉" : "🏅";
      console.log(`${medal} Miner ${miner.index}: ${miner.formatted} DEMLE`);
    });
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  }); 