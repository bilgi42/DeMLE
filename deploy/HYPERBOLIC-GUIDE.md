# 🚀 DEMLE H100 Mining on Hyperbolic

Simple guide for running DEMLE mining on Hyperbolic's H100 GPU instances.

## 📋 Prerequisites

1. **Hyperbolic Account**: Sign up at [hyperbolic.xyz](https://hyperbolic.xyz)
2. **Local Machine/VPS**: To run the blockchain node
3. **Git Repository**: Your DEMLE code accessible from H100 instances
4. **Network Access**: Public IP or tunneling solution (see Step 1 for home networks)

## 🏗️ Simple Architecture

```
Your Machine/VPS                 Hyperbolic Cloud
┌─────────────────────┐         ┌─────────────────────┐
│  Hardhat Node       │◄────────┤  H100 Instance #1   │
│  DEMLE Contract     │         │  (GPU Miner)        │
│  Dashboard          │         └─────────────────────┘
│  Port: 8545         │         ┌─────────────────────┐
└─────────────────────┘    ────►│  H100 Instance #2   │
                                │  (GPU Miner)        │
                                └─────────────────────┘
```

## 🚀 Step-by-Step Setup

### Step 1: Set Up Blockchain (Your Machine)

#### Option A: VPS/Cloud Server (Direct Public IP)
```bash
# 1. Start the blockchain locally
cd DeMLE/contracts
npm run setup-demo

# 2. Note the contract address from output
# 3. Open firewall for port 8545
# Ubuntu/Debian:
sudo ufw allow 8545/tcp
# Fedora/RHEL:
sudo firewall-cmd --add-port=8545/tcp --permanent && sudo firewall-cmd --reload

# 4. Get your public IP
curl ifconfig.me
```

#### Option B: Home Network (Behind NAT) 🏠
If you're on a home internet connection, you'll need a tunnel:

```bash
# 1. Start the blockchain locally
cd DeMLE/contracts
npm run setup-demo

# 2. Note the contract address from output

# 3. Install ngrok (if not already installed)
# Download from: https://ngrok.com/download
# Or on Fedora:
wget https://bin.equinox.io/c/bNyj1mQVY4c/ngrok-v3-stable-linux-amd64.tgz
tar xvf ngrok-v3-stable-linux-amd64.tgz
sudo mv ngrok /usr/local/bin/

# 4. Create free ngrok account at https://ngrok.com
# 5. Get your auth token from dashboard
ngrok authtoken YOUR_AUTH_TOKEN

# 6. Expose your blockchain to the internet
ngrok http 8545
# This will give you a public URL like: https://abc123.ngrok.io
```

**🔥 Important for Home Networks:**
- Your machine likely has a private IP (192.168.x.x or 10.x.x.x)
- The public IP from `curl ifconfig.me` belongs to your router
- Use ngrok to create a secure tunnel to your local blockchain
- Keep the ngrok terminal window open while mining

### Step 2: Launch H100 Instances on Hyperbolic

```bash
# 1. Go to hyperbolic.xyz dashboard
# 2. Create H100 instance:
#    - GPU: H100 (80GB)
#    - vCPUs: 8+
#    - RAM: 32GB+
#    - Storage: 50GB
#    - OS: Ubuntu 22.04

# 3. SSH into the H100 instance
ssh root@<h100-ip>
```

### Step 3: Set Up Mining on H100

```bash
# Install dependencies
apt update && apt install -y curl build-essential git pkg-config libssl-dev

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source ~/.cargo/env

# Clone and build miner
git clone https://github.com/your-username/DeMLE.git
cd DeMLE/demle-miner

# Build with CUDA support
cargo build --release --features cuda

# Start mining
# Option A: Direct public IP
./target/release/demle-miner \
  --contract 0xYourContractAddress \
  --rpc-url http://YOUR_PUBLIC_IP:8545 \
  --threads 16 \
  --target-teraflops 100.0 \
  --verbose

# Option B: ngrok tunnel (for home networks)
./target/release/demle-miner \
  --contract 0xYourContractAddress \
  --rpc-url https://abc123.ngrok.io \
  --threads 16 \
  --target-teraflops 100.0 \
  --verbose
```

## ⚙️ H100 Optimization

### GPU-Optimized Settings

```bash
# For maximum H100 utilization:
export CUDA_VISIBLE_DEVICES=0
export DEMLE_BATCH_SIZE=1024

# Run with aggressive settings
./target/release/demle-miner \
  --contract 0xYourContract \
  --rpc-url http://YOUR_IP:8545 \
  --threads 32 \
  --target-teraflops 200.0 \
  --verbose
```

### Monitor Performance

```bash
# Check GPU utilization
watch -n 1 nvidia-smi

# Monitor mining output
tail -f /path/to/miner/output.log

# Check network connectivity to your blockchain
curl http://YOUR_IP:8545
```

## 📊 Expected Performance

| Instance | GPU Memory | TFLOPS/s | Tokens/Hour | Cost/Hour |
|----------|------------|----------|-------------|-----------|
| H100     | 80GB       | 50-200   | ~7,200      | ~$4-8     |

## 💰 Cost Analysis

```bash
# Example calculation:
# H100 cost: $6/hour
# Mining rate: ~7,200 tokens/hour
# Daily cost: $144
# Daily tokens: ~172,800

# Break-even: Token needs $0.0008+ value
```

## 🔧 Scaling Multiple H100s

### Launch Multiple Instances

```bash
# 1. Create multiple H100 instances on Hyperbolic
# 2. Run the same setup script on each
# 3. All connect to the same blockchain RPC

# Example for 4x H100 fleet:
# Total cost: ~$576/day
# Total tokens: ~691,200/day
# Massive parallel processing power
```

### Auto-Setup Script

```bash
#!/bin/bash
# save as setup-h100.sh

CONTRACT_ADDRESS="0xYourContractAddress"
RPC_URL="http://YOUR_PUBLIC_IP:8545"

# Install everything
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source ~/.cargo/env

git clone https://github.com/your-username/DeMLE.git
cd DeMLE/demle-miner
cargo build --release --features cuda

# Start mining
./target/release/demle-miner \
  --contract $CONTRACT_ADDRESS \
  --rpc-url $RPC_URL \
  --threads 16 \
  --target-teraflops 100.0 \
  --verbose
```

## 🏠 Home Network Troubleshooting

### Check if you're behind NAT:
```bash
# Get your local IP
ip addr show | grep "inet " | grep -v "127.0.0.1"

# Get your public IP
curl ifconfig.me

# If they're different (they usually are), you're behind NAT
```

### Common Issues:

1. **"Connection refused" from H100**
   ```bash
   # Test your tunnel from H100:
   curl https://your-ngrok-url.ngrok.io
   
   # Should return JSON-RPC response
   ```

2. **ngrok session expired**
   ```bash
   # Free ngrok sessions expire after 2 hours
   # Simply restart: ngrok http 8545
   # Or upgrade to paid plan for persistent URLs
   ```

3. **Slow performance over tunnel**
   ```bash
   # This is normal - ngrok adds some latency
   # For production, consider upgrading to paid ngrok
   # Or use a VPS instead
   ```

## 🚀 Quick Start Commands

### For VPS/Cloud Server:
```bash
# 1. Start blockchain (your machine)
npm run setup-demo

# 2. Get contract address from output
# 3. Launch H100 on Hyperbolic
# 4. Run this on H100:
curl -sSL https://raw.githubusercontent.com/your-username/DeMLE/main/scripts/quick-setup.sh | bash -s -- YOUR_CONTRACT_ADDRESS http://YOUR_PUBLIC_IP:8545
```

### For Home Network:
```bash
# 1. Start blockchain (your machine)
npm run setup-demo

# 2. Start ngrok tunnel
ngrok http 8545
# Note the https://xxx.ngrok.io URL

# 3. Launch H100 on Hyperbolic
# 4. Run this on H100:
curl -sSL https://raw.githubusercontent.com/your-username/DeMLE/main/scripts/quick-setup.sh | bash -s -- YOUR_CONTRACT_ADDRESS https://xxx.ngrok.io
```

---

**💡 Key Points:**
- Home networks need ngrok or port forwarding
- VPS/cloud servers can use direct public IP
- ngrok free tier: 2-hour sessions, paid: persistent URLs
- H100s connect over internet to your RPC
- Each H100 can achieve 50-200 TFLOPS
- Start with 1 H100 to test, then scale up 