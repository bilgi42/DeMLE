import { expect } from "chai";
import { ethers } from "hardhat";
import { DEMLE } from "../typechain-types";

describe("DEMLE", function () {
  let demle: DEMLE;
  let owner: any;
  let miner: any;

  beforeEach(async function () {
    [owner, miner] = await ethers.getSigners();
    
    const DEMLE = await ethers.getContractFactory("DEMLE");
    demle = await DEMLE.deploy();
  });

  describe("Deployment", function () {
    it("Should set the right name and symbol", async function () {
      expect(await demle.name()).to.equal("DEMLE");
      expect(await demle.symbol()).to.equal("DEMLE");
    });

    it("Should set the right owner", async function () {
      expect(await demle.owner()).to.equal(owner.address);
    });

    it("Should have correct constants", async function () {
      expect(await demle.MINING_REWARD()).to.equal(ethers.parseEther("100"));
      expect(await demle.MAX_SUPPLY()).to.equal(ethers.parseEther("21000000"));
    });
  });

  describe("Mining", function () {
    it("Should allow valid mining proof submission", async function () {
      const nonce = ethers.randomBytes(32);
      const mlProof = ethers.toUtf8Bytes("test ml proof data");

      await expect(demle.connect(miner).submitMiningProof(nonce, mlProof))
        .to.emit(demle, "MiningReward")
        .withArgs(miner.address, ethers.parseEther("100"), nonce);

      expect(await demle.balanceOf(miner.address)).to.equal(ethers.parseEther("100"));
    });

    it("Should reject reused nonce", async function () {
      const nonce = ethers.randomBytes(32);
      const mlProof = ethers.toUtf8Bytes("test ml proof data");

      await demle.connect(miner).submitMiningProof(nonce, mlProof);
      
      await expect(demle.connect(miner).submitMiningProof(nonce, mlProof))
        .to.be.revertedWith("Nonce already used");
    });

    it("Should reject empty ML proof", async function () {
      const nonce = ethers.randomBytes(32);
      const mlProof = "0x";

      await expect(demle.connect(miner).submitMiningProof(nonce, mlProof))
        .to.be.revertedWith("Invalid ML proof");
    });
  });

  describe("Difficulty", function () {
    it("Should return correct mining difficulty", async function () {
      expect(await demle.getMiningDifficulty()).to.equal(1);
    });
  });
}); 