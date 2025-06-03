import { ethers } from "hardhat";
import { writeFileSync } from "fs";
import express from "express";
import { Server as SocketIOServer } from "socket.io";
import { createServer } from "http";

async function main() {
  console.log("🌐 Starting DEMLE Live Dashboard Server...");

  // Deploy contract for live tracking
  const DEMLE = await ethers.getContractFactory("DEMLE");
  const demle = await DEMLE.deploy();
  await demle.waitForDeployment();
  const contractAddress = await demle.getAddress();

  console.log(`📍 Contract deployed: ${contractAddress}`);
  console.log(`🔗 Network: Hardhat Local (Chain ID: 31337)`);

  // Create Express server with Socket.IO
  const app = express();
  const server = createServer(app);
  const io = new SocketIOServer(server, {
    cors: {
      origin: "*",
      methods: ["GET", "POST"]
    }
  });

  // Serve static files
  app.use(express.static('.'));

  // In-memory storage for tracking miners and events
  let knownMiners = new Map();
  let miningEvents: any[] = [];
  let lastBlockNumber = 0;

  // Function to scan for new miners and events
  async function scanForUpdates() {
    try {
      const currentBlock = await ethers.provider.getBlockNumber();
      
      if (currentBlock > lastBlockNumber) {
        // Get all MiningReward events from the last scanned block
        const filter = demle.filters.MiningReward();
        const events = await demle.queryFilter(filter, lastBlockNumber + 1, currentBlock);
        
        for (const event of events) {
          const args = event.args;
          const minerAddress = args.miner;
          const amount = args.amount;
          const nonce = args.nonce;
          const block = await event.getBlock();
          
          // Add to known miners if new
          if (!knownMiners.has(minerAddress)) {
            const balance = await demle.balanceOf(minerAddress);
            knownMiners.set(minerAddress, {
              address: minerAddress,
              balance: balance,
              lastMining: block.timestamp,
              totalMines: 1,
              name: `Miner ${knownMiners.size + 1}`
            });
          } else {
            // Update existing miner
            const miner = knownMiners.get(minerAddress);
            miner.balance = await demle.balanceOf(minerAddress);
            miner.lastMining = block.timestamp;
            miner.totalMines += 1;
          }
          
          // Add to events log
          miningEvents.unshift({
            id: `${event.transactionHash}-${event.logIndex}`,
            miner: minerAddress,
            amount: ethers.formatEther(amount),
            nonce: nonce,
            timestamp: block.timestamp,
            blockNumber: event.blockNumber,
            transactionHash: event.transactionHash
          });
          
          // Keep only last 100 events
          if (miningEvents.length > 100) {
            miningEvents = miningEvents.slice(0, 100);
          }
        }
        
        lastBlockNumber = currentBlock;
        
        // Emit updates to all connected clients
        if (events.length > 0) {
          await broadcastUpdate();
        }
      }
    } catch (error) {
      console.error('Error scanning for updates:', error);
    }
  }

  // Function to broadcast current state to all clients
  async function broadcastUpdate() {
    try {
      const totalSupply = await demle.totalSupply();
      const maxSupply = await demle.MAX_SUPPLY();
      const miningReward = await demle.MINING_REWARD();
      const difficulty = await demle.getMiningDifficulty();

      const minersArray = Array.from(knownMiners.values()).map((miner: any) => ({
        ...miner,
        balance: ethers.formatEther(miner.balance),
        lastMining: new Date(miner.lastMining * 1000).toISOString()
      }));

      const data = {
        contractAddress,
        totalSupply: ethers.formatEther(totalSupply),
        maxSupply: ethers.formatEther(maxSupply),
        miningReward: ethers.formatEther(miningReward),
        difficulty: difficulty.toString(),
        distribution: ((Number(totalSupply) / Number(maxSupply)) * 100).toFixed(8),
        miners: minersArray,
        recentEvents: miningEvents.slice(0, 10),
        stats: {
          totalMiners: minersArray.length,
          activeMiners: minersArray.filter(m => m.totalMines > 0).length,
          totalMiningEvents: miningEvents.length
        }
      };

      io.emit('dashboardUpdate', data);
      
      // Log update
      console.log(`📊 Update: ${minersArray.length} miners, ${ethers.formatEther(totalSupply)} DEMLE distributed`);
    } catch (error) {
      console.error('Error broadcasting update:', error);
    }
  }

  // Generate dynamic HTML dashboard
  const html = `
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>DEMLE Live Token Distribution Dashboard</title>
    <script src="https://cdn.socket.io/4.7.2/socket.io.min.js"></script>
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
        .miners-section {
            margin: 40px 0;
        }
        .miners-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
            gap: 20px;
            margin: 20px 0;
        }
        .miner-card {
            background: #f8f9fa;
            border: 2px solid #e9ecef;
            border-radius: 12px;
            padding: 20px;
            transition: all 0.3s ease;
        }
        .miner-card.new-mining {
            border-color: #28a745;
            box-shadow: 0 0 20px rgba(40, 167, 69, 0.3);
        }
        .miner-header {
            display: flex;
            align-items: center;
            margin-bottom: 15px;
        }
        .miner-avatar {
            font-size: 2em;
            margin-right: 15px;
        }
        .miner-balance {
            font-size: 1.8em;
            font-weight: bold;
            color: #28a745;
            margin: 10px 0;
        }
        .miner-stats {
            display: grid;
            grid-template-columns: 1fr 1fr;
            gap: 10px;
            margin-top: 15px;
        }
        .miner-stat {
            background: white;
            padding: 10px;
            border-radius: 6px;
            text-align: center;
            border: 1px solid #dee2e6;
        }
        .events-section {
            background: #f8f9fa;
            padding: 20px;
            border-radius: 12px;
            margin: 20px 0;
        }
        .events-log {
            max-height: 400px;
            overflow-y: auto;
            background: white;
            padding: 15px;
            border-radius: 8px;
            border: 1px solid #dee2e6;
        }
        .event-item {
            padding: 12px;
            margin: 8px 0;
            background: #f8f9fa;
            border-radius: 6px;
            border-left: 4px solid #28a745;
            animation: slideIn 0.3s ease;
        }
        .event-item.new {
            background: #d4edda;
            border-left-color: #155724;
        }
        @keyframes slideIn {
            from { opacity: 0; transform: translateX(-20px); }
            to { opacity: 1; transform: translateX(0); }
        }
        .contract-info {
            background: #e9ecef;
            padding: 20px;
            border-radius: 12px;
            margin-top: 40px;
        }
        .address {
            font-family: monospace;
            background: #f8f9fa;
            padding: 8px 12px;
            border-radius: 6px;
            border: 1px solid #dee2e6;
            word-break: break-all;
            cursor: pointer;
        }
        .address:hover {
            background: #e9ecef;
        }
        .instructions {
            background: #d1ecf1;
            border: 1px solid #bee5eb;
            border-radius: 8px;
            padding: 20px;
            margin: 20px 0;
        }
        .chart-container {
            margin: 40px 0;
            background: #f8f9fa;
            padding: 20px;
            border-radius: 12px;
        }
        .no-miners {
            text-align: center;
            padding: 40px;
            color: #6c757d;
            font-style: italic;
        }
        .status-indicator {
            display: inline-block;
            padding: 4px 8px;
            border-radius: 12px;
            font-size: 0.8em;
            font-weight: bold;
        }
        .status-connected {
            background: #d4edda;
            color: #155724;
        }
        .status-disconnected {
            background: #f8d7da;
            color: #721c24;
        }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>🚀 DEMLE Live Dashboard <span class="live-indicator"></span></h1>
            <p>Real-time Token Distribution from ML Mining</p>
            <div class="status-indicator status-connected" id="connectionStatus">Connected</div>
        </div>

        <div class="stats-grid">
            <div class="stat-card">
                <h3>Total Supply</h3>
                <div class="value" id="totalSupply">0</div>
                <p>DEMLE Tokens</p>
            </div>
            <div class="stat-card">
                <h3>Active Miners</h3>
                <div class="value" id="activeMiners">0</div>
                <p>Currently Mining</p>
            </div>
            <div class="stat-card">
                <h3>Mining Reward</h3>
                <div class="value" id="miningReward">100</div>
                <p>DEMLE per Proof</p>
            </div>
            <div class="stat-card">
                <h3>Difficulty</h3>
                <div class="value" id="difficulty">1</div>
                <p>Current Level</p>
            </div>
            <div class="stat-card">
                <h3>Distribution</h3>
                <div class="value" id="distribution">0.0000%</div>
                <p>Of Max Supply</p>
            </div>
        </div>

        <div class="instructions">
            <h3>🔗 Connect Your Miner</h3>
            <p><strong>Contract Address:</strong></p>
            <div class="address" id="contractAddress" onclick="copyToClipboard(this.textContent)">${contractAddress}</div>
            <p><strong>Network:</strong> Hardhat Local (localhost:8545, Chain ID: 31337)</p>
            <p><strong>To connect your Rust miner:</strong></p>
            <pre>cargo run --bin demle-miner -- --contract ${contractAddress} --rpc http://localhost:8545</pre>
        </div>

        <div class="miners-section">
            <h2>👥 Active Miners (<span id="minerCount">0</span>)</h2>
            <div class="miners-grid" id="minersGrid">
                <div class="no-miners">
                    No miners detected yet. Start your miner to see it appear here!
                </div>
            </div>
        </div>

        <div class="chart-container" id="chartContainer" style="display: none;">
            <h3>📊 Token Distribution Chart</h3>
            <canvas id="tokenChart" width="400" height="200"></canvas>
        </div>

        <div class="events-section">
            <h3>⛏️ Live Mining Events</h3>
            <div class="events-log" id="eventsLog">
                <div class="no-miners">Waiting for mining activity...</div>
            </div>
        </div>

        <div class="contract-info">
            <h3>📋 Contract Information</h3>
            <p><strong>Network:</strong> Hardhat Local (Chain ID: 31337)</p>
            <p><strong>Token Standard:</strong> ERC-20</p>
            <p><strong>Mining Mechanism:</strong> FP8 ML Computation Proof-of-Work</p>
            <p><strong>Max Supply:</strong> 21,000,000 DEMLE</p>
        </div>
    </div>

    <script>
        const socket = io('http://localhost:3000');
        let chart = null;
        let lastEventIds = new Set();

        socket.on('connect', () => {
            console.log('Connected to dashboard server');
            document.getElementById('connectionStatus').textContent = 'Connected';
            document.getElementById('connectionStatus').className = 'status-indicator status-connected';
        });

        socket.on('disconnect', () => {
            console.log('Disconnected from dashboard server');
            document.getElementById('connectionStatus').textContent = 'Disconnected';
            document.getElementById('connectionStatus').className = 'status-indicator status-disconnected';
        });

        socket.on('dashboardUpdate', (data) => {
            updateDashboard(data);
        });

        function updateDashboard(data) {
            // Update stats
            document.getElementById('totalSupply').textContent = data.totalSupply;
            document.getElementById('activeMiners').textContent = data.stats.activeMiners;
            document.getElementById('miningReward').textContent = data.miningReward;
            document.getElementById('difficulty').textContent = data.difficulty;
            document.getElementById('distribution').textContent = data.distribution + '%';
            document.getElementById('contractAddress').textContent = data.contractAddress;
            document.getElementById('minerCount').textContent = data.miners.length;

            // Update miners
            updateMiners(data.miners);

            // Update chart
            updateChart(data.miners);

            // Update events
            updateEvents(data.recentEvents);
        }

        function updateMiners(miners) {
            const minersGrid = document.getElementById('minersGrid');
            
            if (miners.length === 0) {
                minersGrid.innerHTML = '<div class="no-miners">No miners detected yet. Start your miner to see it appear here!</div>';
                return;
            }

            minersGrid.innerHTML = miners.map((miner, index) => \`
                <div class="miner-card" id="miner-\${miner.address}">
                    <div class="miner-header">
                        <div class="miner-avatar">\${getMinerAvatar(index)}</div>
                        <div class="miner-info">
                            <h4>\${miner.name}</h4>
                            <small>\${miner.address.slice(0, 8)}...\${miner.address.slice(-6)}</small>
                        </div>
                    </div>
                    <div class="miner-balance">\${miner.balance} DEMLE</div>
                    <div class="miner-stats">
                        <div class="miner-stat">
                            <strong>\${miner.totalMines}</strong><br>
                            <small>Total Mines</small>
                        </div>
                        <div class="miner-stat">
                            <strong>\${new Date(miner.lastMining).toLocaleTimeString()}</strong><br>
                            <small>Last Mining</small>
                        </div>
                    </div>
                </div>
            \`).join('');
        }

        function updateChart(miners) {
            const chartContainer = document.getElementById('chartContainer');
            
            if (miners.length === 0) {
                chartContainer.style.display = 'none';
                return;
            }

            chartContainer.style.display = 'block';
            
            const ctx = document.getElementById('tokenChart').getContext('2d');
            
            if (chart) {
                chart.destroy();
            }

            chart = new Chart(ctx, {
                type: 'doughnut',
                data: {
                    labels: miners.map(m => m.name),
                    datasets: [{
                        label: 'DEMLE Token Holdings',
                        data: miners.map(m => parseFloat(m.balance)),
                        backgroundColor: [
                            'rgba(255, 99, 132, 0.8)',
                            'rgba(54, 162, 235, 0.8)',
                            'rgba(255, 205, 86, 0.8)',
                            'rgba(75, 192, 192, 0.8)',
                            'rgba(153, 102, 255, 0.8)',
                            'rgba(255, 159, 64, 0.8)'
                        ],
                        borderWidth: 2
                    }]
                },
                options: {
                    responsive: true,
                    plugins: {
                        legend: { position: 'bottom' },
                        title: {
                            display: true,
                            text: 'Live DEMLE Token Distribution'
                        }
                    }
                }
            });
        }

        function updateEvents(events) {
            const eventsLog = document.getElementById('eventsLog');
            
            if (events.length === 0) {
                eventsLog.innerHTML = '<div class="no-miners">Waiting for mining activity...</div>';
                return;
            }

            eventsLog.innerHTML = events.map(event => {
                const isNew = !lastEventIds.has(event.id);
                if (isNew) lastEventIds.add(event.id);
                
                return \`
                    <div class="event-item \${isNew ? 'new' : ''}">
                        <strong>Mining Success!</strong><br>
                        <small>Miner: \${event.miner.slice(0, 8)}...\${event.miner.slice(-6)}</small><br>
                        <small>Earned: \${event.amount} DEMLE</small><br>
                        <small>Time: \${new Date(event.timestamp * 1000).toLocaleString()}</small><br>
                        <small>TX: \${event.transactionHash.slice(0, 10)}...</small>
                    </div>
                \`;
            }).join('');
        }

        function getMinerAvatar(index) {
            const avatars = ['👩‍💻', '👨‍🔬', '🧑‍💼', '👩‍🚀', '👨‍💻', '🧑‍🔬'];
            return avatars[index % avatars.length];
        }

        function copyToClipboard(text) {
            navigator.clipboard.writeText(text).then(() => {
                alert('Contract address copied to clipboard!');
            });
        }

        // Cleanup chart on page unload
        window.addEventListener('beforeunload', () => {
            if (chart) chart.destroy();
        });
    </script>
</body>
</html>
  `;

  // Write the HTML file
  writeFileSync('./live-dashboard.html', html);

  // Socket.IO connection handling
  io.on('connection', (socket) => {
    console.log('Client connected to dashboard');
    
    // Send initial state immediately
    broadcastUpdate();
    
    socket.on('disconnect', () => {
      console.log('Client disconnected from dashboard');
    });
  });

  // Start periodic scanning for updates
  console.log('📡 Starting blockchain monitor...');
  setInterval(scanForUpdates, 2000); // Check every 2 seconds

  // API endpoint for contract info
  app.get('/api/contract', (req, res) => {
    res.json({
      address: contractAddress,
      network: 'hardhat',
      chainId: 31337
    });
  });

  // Start server
  const PORT = 3000;
  server.listen(PORT, () => {
    console.log(\`\n🎉 DEMLE Live Dashboard is running!\`);
    console.log(\`📍 Dashboard: http://localhost:\${PORT}/live-dashboard.html\`);
    console.log(\`📍 Contract: \${contractAddress}\`);
    console.log(\`🔗 Network: Hardhat Local (localhost:8545)\`);
    console.log(\`\n🚀 Connect your miners and watch the dashboard update live!\`);
    console.log(\`   cargo run --bin demle-miner -- --contract \${contractAddress} --rpc http://localhost:8545\`);
  });
}

main().catch((error) => {
  console.error(error);
  process.exit(1);
}); 