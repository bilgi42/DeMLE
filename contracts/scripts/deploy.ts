import { ethers } from "hardhat";

async function main() {
  console.log("Deploying DEMLE contract...");

  const [deployer] = await ethers.getSigners();
  console.log("Deploying with account:", deployer.address);
  console.log("Account balance:", (await ethers.provider.getBalance(deployer.address)).toString());

  const DEMLE = await ethers.getContractFactory("DEMLE");
  const demle = await DEMLE.deploy();

  await demle.waitForDeployment();
  const address = await demle.getAddress();

  console.log("DEMLE deployed to:", address);
  console.log("Mining reward:", await demle.MINING_REWARD());
  console.log("Max supply:", await demle.MAX_SUPPLY());
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  }); 