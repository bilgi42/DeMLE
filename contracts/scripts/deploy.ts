import { ethers } from "hardhat";

async function main() {
  console.log("🚀 Deploying DEMLE contract...");

  const [deployer] = await ethers.getSigners();
  console.log("🏗️  Deploying with account:", deployer.address);
  console.log("💰 Account balance:", ethers.formatEther(await ethers.provider.getBalance(deployer.address)), "ETH");

  const DEMLE = await ethers.getContractFactory("DEMLE");
  console.log("📦 Deploying contract...");
  
  const demle = await DEMLE.deploy();
  await demle.waitForDeployment();
  
  const address = await demle.getAddress();
  
  console.log("\n✅ DEMLE Successfully Deployed!");
  console.log("=" .repeat(50));
  console.log("📍 Contract Address:", address);
  console.log("⛽ Mining Reward:", ethers.formatEther(await demle.MINING_REWARD()), "DEMLE");
  console.log("🏦 Max Supply:", ethers.formatEther(await demle.MAX_SUPPLY()), "DEMLE");
  console.log("⚡ Initial Difficulty:", await demle.getMiningDifficulty());
  
  console.log("\n🚀 Ready to mine! Use this command:");
  console.log(`cargo run --bin demle-miner -- --contract ${address} --rpc-url http://localhost:8545`);
  
  console.log("\n📊 Check balances with:");
  console.log(`npx hardhat run scripts/check-real-balances.ts --network localhost`);
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  }); 