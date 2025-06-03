// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

/**
 * @title DEMLE Token
 * @dev ERC20 token for rewarding FP8 ML mining computations
 */
contract DEMLE is ERC20, Ownable {
    uint256 public constant MINING_REWARD = 100 * 10**18; // 100 DEMLE tokens
    uint256 public constant MAX_SUPPLY = 21_000_000 * 10**18; // 21 million total supply
    
    mapping(address => uint256) public lastMiningTime;
    mapping(bytes32 => bool) public usedNonces;
    
    event MiningReward(address indexed miner, uint256 amount, bytes32 nonce);
    
    constructor() ERC20("DEMLE", "DEMLE") Ownable(msg.sender) {}
    
    /**
     * @dev Verify FP8 ML computation proof and reward miner
     * @param nonce Unique computation identifier
     * @param mlProof Proof of ML computation (simplified for demo)
     */
    function submitMiningProof(bytes32 nonce, bytes calldata mlProof) external {
        require(!usedNonces[nonce], "Nonce already used");
        require(mlProof.length > 0, "Invalid ML proof");
        require(totalSupply() + MINING_REWARD <= MAX_SUPPLY, "Max supply reached");
        
        // Simple proof verification (replace with actual FP8 ML verification)
        require(_verifyMLProof(nonce, mlProof), "Invalid ML proof");
        
        usedNonces[nonce] = true;
        lastMiningTime[msg.sender] = block.timestamp;
        
        _mint(msg.sender, MINING_REWARD);
        
        emit MiningReward(msg.sender, MINING_REWARD, nonce);
    }
    
    /**
     * @dev Simplified ML proof verification
     * In production, this would verify FP8 matrix operations, convolutions, etc.
     */
    function _verifyMLProof(bytes32 nonce, bytes calldata mlProof) internal pure returns (bool) {
        // Simplified verification: check if proof contains nonce
        bytes32 proofHash = keccak256(mlProof);
        return proofHash != bytes32(0) && nonce != bytes32(0);
    }
    
    /**
     * @dev Get mining difficulty (simplified)
     */
    function getMiningDifficulty() external view returns (uint256) {
        // Adjust difficulty based on total supply
        uint256 supply = totalSupply();
        if (supply < 1_000_000 * 10**18) return 1;
        if (supply < 5_000_000 * 10**18) return 2;
        if (supply < 10_000_000 * 10**18) return 3;
        return 4;
    }
} 