import { ethers } from "hardhat";
import { writeFileSync } from "fs";

async function main() {
  console.log("🌐 Generating DEMLE Token Dashboard...");

  // Deploy contract and simulate mining
  const DEMLE = await ethers.getContractFactory("DEMLE");
  const demle = await DEMLE.deploy();
  await demle.waitForDeployment();
  const contractAddress = await demle.getAddress();

  const [owner, miner1, miner2, miner3, miner4] = await ethers.getSigners();
  const miners = [
    { signer: miner1, name: "Alice", avatar: "👩‍💻" },
    { signer: miner2, name: "Bob", avatar: "👨‍🔬" },
    { signer: miner3, name: "Charlie", avatar: "🧑‍💼" },
    { signer: miner4, name: "Diana", avatar: "👩‍🚀" }
  ];

  // Simulate varied mining activity
  console.log("⛏️  Simulating mining activity...");
  const miningResults = [];

  for (let round = 1; round <= 5; round++) {
    for (let i = 0; i < miners.length; i++) {
      // Not all miners succeed each round (simulating real conditions)
      if (Math.random() > 0.3) { // 70% success rate
        const miner = miners[i];
        const nonce = ethers.randomBytes(32);
        const mlProof = ethers.toUtf8Bytes(JSON.stringify({
          round: round,
          operation: ["gemm", "conv2d", "attention", "batchnorm"][i % 4],
          precision: "FP8_E4M3",
          timestamp: Date.now()
        }));

        try {
          const tx = await demle.connect(miner.signer).submitMiningProof(nonce, mlProof);
          await tx.wait();
          miningResults.push({
            round,
            miner: miner.name,
            success: true,
            operation: ["gemm", "conv2d", "attention", "batchnorm"][i % 4]
          });
        } catch {
          miningResults.push({
            round,
            miner: miner.name,
            success: false,
            operation: ["gemm", "conv2d", "attention", "batchnorm"][i % 4]
          });
        }
      }
    }
  }

  // Collect data
  const totalSupply = await demle.totalSupply();
  const maxSupply = await demle.MAX_SUPPLY();
  const miningReward = await demle.MINING_REWARD();

  const minerData = [];
  for (const miner of miners) {
    const balance = await demle.balanceOf(miner.signer.address);
    const lastMining = await demle.lastMiningTime(miner.signer.address);
    
    minerData.push({
      name: miner.name,
      avatar: miner.avatar,
      address: miner.signer.address,
      balance: ethers.formatEther(balance),
      balanceWei: balance.toString(),
      lastMining: lastMining.toString(),
      successfulMines: miningResults.filter(r => r.miner === miner.name && r.success).length,
      totalAttempts: miningResults.filter(r => r.miner === miner.name).length
    });
  }

  // Generate HTML dashboard
  const html = `
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>DEMLE Token Distribution Dashboard</title>
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
            max-width: 1200px;
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
        .header p {
            margin: 10px 0 0 0;
            color: #666;
            font-size: 1.1em;
        }
        .stats-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
            gap: 20px;
            margin-bottom: 40px;
        }
        .stat-card {
            background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%);
            padding: 25px;
            border-radius: 12px;
            color: white;
            text-align: center;
            box-shadow: 0 8px 16px rgba(0,0,0,0.1);
        }
        .stat-card h3 {
            margin: 0 0 10px 0;
            font-size: 1.1em;
            opacity: 0.9;
        }
        .stat-card .value {
            font-size: 2em;
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
            transition: transform 0.2s;
        }
        .miner-card:hover {
            transform: translateY(-5px);
            box-shadow: 0 12px 24px rgba(0,0,0,0.1);
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
        .miner-info h4 {
            margin: 0;
            color: #495057;
            font-size: 1.3em;
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
        .chart-container {
            margin: 40px 0;
            background: #f8f9fa;
            padding: 20px;
            border-radius: 12px;
        }
        .contract-info {
            background: #e9ecef;
            padding: 20px;
            border-radius: 12px;
            margin-top: 40px;
        }
        .contract-info h3 {
            color: #495057;
            margin-top: 0;
        }
        .address {
            font-family: monospace;
            background: #f8f9fa;
            padding: 8px 12px;
            border-radius: 6px;
            border: 1px solid #dee2e6;
            word-break: break-all;
        }
        .mining-activity {
            background: #f8f9fa;
            padding: 20px;
            border-radius: 12px;
            margin: 20px 0;
        }
        .activity-log {
            max-height: 300px;
            overflow-y: auto;
            background: white;
            padding: 15px;
            border-radius: 8px;
            border: 1px solid #dee2e6;
        }
        .activity-item {
            padding: 8px 0;
            border-bottom: 1px solid #f1f3f4;
            display: flex;
            justify-content: space-between;
            align-items: center;
        }
        .activity-item:last-child {
            border-bottom: none;
        }
        .success { color: #28a745; }
        .failure { color: #dc3545; }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>🚀 DEMLE Token Distribution</h1>
            <p>Decentralized Machine Learning Efforts - Live Token Analytics</p>
        </div>

        <div class="stats-grid">
            <div class="stat-card">
                <h3>Total Supply</h3>
                <div class="value">${ethers.formatEther(totalSupply)}</div>
                <p>DEMLE Tokens</p>
            </div>
            <div class="stat-card">
                <h3>Max Supply</h3>
                <div class="value">${(Number(ethers.formatEther(maxSupply)) / 1000000).toFixed(1)}M</div>
                <p>DEMLE Tokens</p>
            </div>
            <div class="stat-card">
                <h3>Mining Reward</h3>
                <div class="value">${ethers.formatEther(miningReward)}</div>
                <p>Per ML Proof</p>
            </div>
            <div class="stat-card">
                <h3>Distribution</h3>
                <div class="value">${((Number(totalSupply) / Number(maxSupply)) * 100).toFixed(4)}%</div>
                <p>Of Max Supply</p>
            </div>
        </div>

        <div class="miners-section">
            <h2>👥 Miner Token Holdings</h2>
            <div class="miners-grid">
                ${minerData.map(miner => `
                <div class="miner-card">
                    <div class="miner-header">
                        <div class="miner-avatar">${miner.avatar}</div>
                        <div class="miner-info">
                            <h4>${miner.name}</h4>
                            <small>${miner.address.slice(0, 8)}...${miner.address.slice(-6)}</small>
                        </div>
                    </div>
                    <div class="miner-balance">${miner.balance} DEMLE</div>
                    <div class="miner-stats">
                        <div class="miner-stat">
                            <strong>${miner.successfulMines}</strong><br>
                            <small>Successful Mines</small>
                        </div>
                        <div class="miner-stat">
                            <strong>${miner.totalAttempts > 0 ? Math.round((miner.successfulMines / miner.totalAttempts) * 100) : 0}%</strong><br>
                            <small>Success Rate</small>
                        </div>
                    </div>
                </div>
                `).join('')}
            </div>
        </div>

        <div class="chart-container">
            <h3>📊 Token Distribution Chart</h3>
            <canvas id="tokenChart" width="400" height="200"></canvas>
        </div>

        <div class="mining-activity">
            <h3>⛏️ Mining Activity Log</h3>
            <div class="activity-log">
                ${miningResults.map((result, index) => `
                <div class="activity-item">
                    <span>Round ${result.round}: ${result.miner} - ${result.operation}</span>
                    <span class="${result.success ? 'success' : 'failure'}">
                        ${result.success ? '✅ Success' : '❌ Failed'}
                    </span>
                </div>
                `).join('')}
            </div>
        </div>

        <div class="contract-info">
            <h3>🔗 Contract Information</h3>
            <p><strong>Contract Address:</strong></p>
            <div class="address">${contractAddress}</div>
            <p><strong>Network:</strong> Hardhat Local (Chain ID: 31337)</p>
            <p><strong>Token Standard:</strong> ERC-20</p>
            <p><strong>Mining Mechanism:</strong> FP8 ML Computation Proof-of-Work</p>
        </div>
    </div>

    <script>
        // Token distribution chart
        const ctx = document.getElementById('tokenChart').getContext('2d');
        const chartData = {
            labels: [${minerData.map(m => `'${m.name}'`).join(', ')}],
            datasets: [{
                label: 'DEMLE Token Holdings',
                data: [${minerData.map(m => m.balance).join(', ')}],
                backgroundColor: [
                    'rgba(255, 99, 132, 0.8)',
                    'rgba(54, 162, 235, 0.8)',
                    'rgba(255, 205, 86, 0.8)',
                    'rgba(75, 192, 192, 0.8)'
                ],
                borderColor: [
                    'rgba(255, 99, 132, 1)',
                    'rgba(54, 162, 235, 1)',
                    'rgba(255, 205, 86, 1)',
                    'rgba(75, 192, 192, 1)'
                ],
                borderWidth: 2
            }]
        };

        new Chart(ctx, {
            type: 'doughnut',
            data: chartData,
            options: {
                responsive: true,
                plugins: {
                    legend: {
                        position: 'bottom'
                    },
                    title: {
                        display: true,
                        text: 'DEMLE Token Distribution Among Miners'
                    }
                }
            }
        });

        // Auto-refresh notice
        console.log('DEMLE Dashboard loaded successfully!');
        console.log('Contract Address: ${contractAddress}');
        console.log('Total Supply: ${ethers.formatEther(totalSupply)} DEMLE');
    </script>
</body>
</html>
  `;

  // Write HTML file
  writeFileSync('./demle-dashboard.html', html);
  
  console.log("✅ Dashboard generated successfully!");
  console.log("📁 File: demle-dashboard.html");
  console.log("🌐 Open the HTML file in your browser to view the dashboard");
  console.log(`📊 Shows distribution of ${ethers.formatEther(totalSupply)} DEMLE tokens among ${miners.length} miners`);
  console.log(`📍 Contract: ${contractAddress}`);
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  }); 