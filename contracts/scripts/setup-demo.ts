import { ethers } from "hardhat";
import { writeFileSync } from "fs";

async function main() {
  console.log("🚀 Setting up DEMLE Demo Environment...");
  console.log("=" .repeat(60));

  // Deploy contract first
  const [deployer] = await ethers.getSigners();
  console.log("🏗️  Deploying with account:", deployer.address);
  console.log("💰 Account balance:", ethers.formatEther(await ethers.provider.getBalance(deployer.address)), "ETH");

  const DEMLE = await ethers.getContractFactory("DEMLE");
  console.log("📦 Deploying DEMLE contract...");
  
  const demle = await DEMLE.deploy();
  await demle.waitForDeployment();
  const contractAddress = await demle.getAddress();

  console.log("\n✅ Contract Deployed Successfully!");
  console.log("📍 Contract Address:", contractAddress);
  console.log("⛽ Mining Reward:", ethers.formatEther(await demle.MINING_REWARD()), "DEMLE");
  console.log("🏦 Max Supply:", ethers.formatEther(await demle.MAX_SUPPLY()), "DEMLE");
  console.log("⚡ Initial Difficulty:", await demle.getMiningDifficulty());

  // Create the dashboard HTML
  const html = `
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>DEMLE Real-time Token Distribution</title>
    <script src="https://cdn.ethers.io/lib/ethers-5.7.2.umd.min.js"></script>
    <script src="https://cdn.jsdelivr.net/npm/chart.js"></script>
    <style>
        body {
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            margin: 0;
            padding: 20px;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: #333;
            min-height: 100vh;
        }
        .container {
            max-width: 1400px;
            margin: 0 auto;
            background: white;
            border-radius: 15px;
            padding: 30px;
            box-shadow: 0 20px 40px rgba(0,0,0,0.1);
        }
        .header {
            text-align: center;
            margin-bottom: 40px;
            padding-bottom: 20px;
            border-bottom: 3px solid #667eea;
        }
        .header h1 {
            margin: 0;
            color: #667eea;
            font-size: 2.5em;
            font-weight: 700;
        }
        .live-indicator {
            display: inline-block;
            width: 12px;
            height: 12px;
            background: #28a745;
            border-radius: 50%;
            margin-left: 10px;
            animation: pulse 2s infinite;
        }
        @keyframes pulse {
            0% { opacity: 1; }
            50% { opacity: 0.5; }
            100% { opacity: 1; }
        }
        .setup-info {
            background: #d1ecf1;
            border: 1px solid #bee5eb;
            border-radius: 8px;
            padding: 20px;
            margin: 20px 0;
        }
        .address {
            font-family: monospace;
            background: #f8f9fa;
            padding: 8px 12px;
            border-radius: 6px;
            border: 1px solid #dee2e6;
            word-break: break-all;
            cursor: pointer;
            margin: 5px 0;
        }
        .address:hover {
            background: #e9ecef;
        }
        .stats-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 20px;
            margin-bottom: 40px;
        }
        .stat-card {
            background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%);
            padding: 20px;
            border-radius: 12px;
            color: white;
            text-align: center;
            box-shadow: 0 8px 16px rgba(0,0,0,0.1);
        }
        .stat-card h3 {
            margin: 0 0 10px 0;
            font-size: 1em;
            opacity: 0.9;
        }
        .stat-card .value {
            font-size: 1.8em;
            font-weight: bold;
            margin: 10px 0;
        }
        .command-box {
            background: #2d3748;
            color: #e2e8f0;
            padding: 15px;
            border-radius: 8px;
            font-family: monospace;
            margin: 10px 0;
            overflow-x: auto;
            white-space: pre-wrap;
        }
        .miners-section {
            margin: 40px 0;
        }
        .no-miners {
            text-align: center;
            padding: 40px;
            color: #6c757d;
            font-style: italic;
        }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>🌟 DEMLE Real-time Dashboard <span class="live-indicator"></span></h1>
            <p>Live ML Mining Token Distribution</p>
        </div>

        <div class="setup-info">
            <h3>📍 Contract Information</h3>
            <p><strong>Contract Address:</strong> <span class="address" onclick="copyToClipboard('${contractAddress}')">${contractAddress}</span></p>
            <p><strong>Network:</strong> Hardhat Local (localhost:8545, Chain ID: 31337)</p>
            <p><strong>Command to run your Rust miner:</strong></p>
            <div class="command-box">cargo run --bin demle-miner -- --contract ${contractAddress} --rpc-url http://localhost:8545</div>
            <p><em>💡 Click the contract address to copy it!</em></p>
        </div>

        <div class="stats-grid">
            <div class="stat-card">
                <h3>💰 Total Supply</h3>
                <div class="value" id="totalSupply">0 DEMLE</div>
            </div>
            <div class="stat-card">
                <h3>⛏️ Mining Reward</h3>
                <div class="value" id="miningReward">100 DEMLE</div>
            </div>
            <div class="stat-card">
                <h3>👥 Active Miners</h3>
                <div class="value" id="activeMiners">0</div>
            </div>
            <div class="stat-card">
                <h3>📊 Latest Block</h3>
                <div class="value" id="latestBlock">0</div>
            </div>
        </div>

        <div class="miners-section">
            <h2>⛏️ Active Miners</h2>
            <div id="minersGrid" class="no-miners">
                Waiting for miners... Start your miner to see it appear here!<br>
                <small>Dashboard updates every 3 seconds</small>
            </div>
        </div>
    </div>

    <script>
        const CONTRACT_ADDRESS = '${contractAddress}';
        const RPC_URL = 'http://localhost:8545';
        
        const CONTRACT_ABI = [
            "function totalSupply() view returns (uint256)",
            "function MINING_REWARD() view returns (uint256)",
            "function balanceOf(address) view returns (uint256)",
            "function lastMiningTime(address) view returns (uint256)",
            "event MiningReward(address indexed miner, uint256 amount, bytes32 nonce)"
        ];

        let provider;
        let contract;
        let knownMiners = new Map();

        async function init() {
            try {
                provider = new ethers.providers.JsonRpcProvider(RPC_URL);
                contract = new ethers.Contract(CONTRACT_ADDRESS, CONTRACT_ABI, provider);
                
                console.log('Connected to contract:', CONTRACT_ADDRESS);
                
                // Start monitoring
                startMonitoring();
                
            } catch (error) {
                console.error('Initialization error:', error);
                document.getElementById('minersGrid').innerHTML = 
                    '<div class="no-miners">❌ Connection Error: Make sure Hardhat node is running!</div>';
            }
        }

        function startMonitoring() {
            // Update dashboard every 3 seconds
            setInterval(updateDashboard, 3000);
            updateDashboard(); // Initial update
        }

        async function updateDashboard() {
            try {
                const blockNumber = await provider.getBlockNumber();
                const totalSupply = await contract.totalSupply();
                const miningReward = await contract.MINING_REWARD();
                
                document.getElementById('latestBlock').textContent = blockNumber;
                document.getElementById('totalSupply').textContent = 
                    ethers.utils.formatEther(totalSupply) + ' DEMLE';
                document.getElementById('miningReward').textContent = 
                    ethers.utils.formatEther(miningReward) + ' DEMLE';
                document.getElementById('activeMiners').textContent = knownMiners.size;
                
                // Scan for miners (check first 20 accounts)
                const accounts = [];
                for (let i = 0; i < 20; i++) {
                    try {
                        const address = ethers.utils.computeAddress(\`0x\${i.toString().padStart(64, '0')}\`);
                        accounts.push(address);
                    } catch (e) {
                        // Skip invalid addresses
                    }
                }
                
                // Add hardhat default accounts
                const defaultAccounts = [
                    '0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266',
                    '0x70997970C51812dc3A010C7d01b50e0d17dc79C8',
                    '0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC',
                    '0x90F79bf6EB2c4f870365E785982E1f101E93b906',
                    '0x15d34AAf54267DB7D7c367839AAf71A00a2C6A65'
                ];
                
                for (const address of defaultAccounts) {
                    try {
                        const balance = await contract.balanceOf(address);
                        const lastMiningTime = await contract.lastMiningTime(address);
                        
                        if (balance.gt(0) || lastMiningTime.gt(0)) {
                            knownMiners.set(address, {
                                address: address,
                                balance: balance,
                                lastMiningTime: lastMiningTime,
                                name: \`Miner \${knownMiners.size + 1}\`
                            });
                        }
                    } catch (e) {
                        console.warn('Error checking address:', address, e);
                    }
                }
                
                updateMinersDisplay();
                
            } catch (error) {
                console.error('Dashboard update error:', error);
            }
        }

        function updateMinersDisplay() {
            const minersGrid = document.getElementById('minersGrid');
            
            if (knownMiners.size === 0) {
                minersGrid.innerHTML = \`
                    <div class="no-miners">
                        Waiting for miners... Start your miner to see it appear here!<br>
                        <small>Dashboard updates every 3 seconds</small>
                    </div>
                \`;
                return;
            }
            
            const miners = Array.from(knownMiners.values());
            minersGrid.innerHTML = miners.map((miner, index) => \`
                <div class="miner-card">
                    <h4>⛏️ \${miner.name}</h4>
                    <p><strong>Address:</strong> \${miner.address.slice(0, 10)}...\${miner.address.slice(-8)}</p>
                    <p><strong>Balance:</strong> \${ethers.utils.formatEther(miner.balance)} DEMLE</p>
                    <p><strong>Last Mining:</strong> \${
                        miner.lastMiningTime.gt(0) 
                            ? new Date(miner.lastMiningTime.toNumber() * 1000).toLocaleString()
                            : 'Never'
                    }</p>
                </div>
            \`).join('');
        }

        function copyToClipboard(text) {
            navigator.clipboard.writeText(text).then(() => {
                alert('Contract address copied to clipboard! 📋');
            }).catch(() => {
                const el = document.createElement('textarea');
                el.value = text;
                document.body.appendChild(el);
                el.select();
                document.execCommand('copy');
                document.body.removeChild(el);
                alert('Contract address copied to clipboard! 📋');
            });
        }

        window.addEventListener('load', init);
    </script>
</body>
</html>`;

  // Write the HTML file
  writeFileSync('./demle-dashboard.html', html);

  console.log("\n✅ Demo Environment Setup Complete!");
  console.log("=" .repeat(60));
  console.log("📁 Dashboard: demle-dashboard.html");
  console.log("🌐 Open the dashboard file in your browser");
  console.log("📍 Contract:", contractAddress);
  console.log("🔗 Network: Hardhat Local (localhost:8545)");

  console.log("\n🚀 Quick Start Guide:");
  console.log("1. Open demle-dashboard.html in your browser");
  console.log("2. Keep Hardhat node running: npx hardhat node");
  console.log("3. Start mining:");
  console.log(`   cargo run --bin demle-miner -- --contract ${contractAddress} --rpc-url http://localhost:8545`);
  console.log("4. Watch the dashboard update live!");

  console.log("\n📊 Monitor with:");
  console.log("   npx hardhat run scripts/check-real-balances.ts --network localhost");

  // Keep the script running to maintain the network
  console.log("\n🔄 Keeping script running to maintain network...");
  console.log("   Press Ctrl+C to stop");
  
  // Simple monitoring loop
  setInterval(async () => {
    try {
      const totalSupply = await demle.totalSupply();
      const blockNumber = await ethers.provider.getBlockNumber();
      
      if (Number(totalSupply) > 0) {
        console.log(`📊 Block ${blockNumber}: ${ethers.formatEther(totalSupply)} DEMLE distributed`);
      }
    } catch (error) {
      console.error('Monitor error:', error);
    }
  }, 10000); // Log every 10 seconds
}

main().catch((error) => {
  console.error(error);
  process.exit(1);
}); 