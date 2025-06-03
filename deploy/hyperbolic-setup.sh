#!/bin/bash

# DEMLE H100 Mining Setup for Hyperbolic
# Usage: ./hyperbolic-setup.sh <coordinator_ip> <contract_address>

set -e

COORDINATOR_IP=${1:-"localhost"}
CONTRACT_ADDRESS=${2}
INSTANCE_COUNT=${3:-4}

echo "🚀 Setting up DEMLE H100 Mining on Hyperbolic"
echo "📍 Coordinator: $COORDINATOR_IP"
echo "🏭 Contract: $CONTRACT_ADDRESS"
echo "⚙️  Instances: $INSTANCE_COUNT"

# Function to set up coordinator
setup_coordinator() {
    echo "🛠️  Setting up coordinator instance..."
    
    # Install Node.js and dependencies
    curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
    sudo apt-get install -y nodejs
    
    # Clone and setup project
    git clone https://github.com/your-username/DeMLE.git
    cd DeMLE/contracts
    npm install
    
    # Start blockchain (in background)
    nohup npm run setup-demo > blockchain.log 2>&1 &
    
    # Wait for blockchain to start
    sleep 30
    
    # Extract contract address from logs
    CONTRACT=$(grep "Contract Address:" blockchain.log | awk '{print $4}')
    echo "✅ Coordinator ready with contract: $CONTRACT"
    echo "$CONTRACT" > /tmp/contract_address
}

# Function to set up H100 miner
setup_h100_miner() {
    local miner_id=$1
    echo "⛏️  Setting up H100 miner #$miner_id..."
    
    # Install CUDA drivers if not present
    if ! command -v nvcc &> /dev/null; then
        wget https://developer.download.nvidia.com/compute/cuda/12.2.0/local_installers/cuda_12.2.0_535.54.03_linux.run
        sudo sh cuda_12.2.0_535.54.03_linux.run --silent --toolkit
    fi
    
    # Install Rust
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source ~/.cargo/env
    
    # Clone project
    git clone https://github.com/your-username/DeMLE.git
    cd DeMLE
    
    # Build optimized miner
    cd demle-miner
    CUDA_HOME=/usr/local/cuda cargo build --release --features cuda
    
    # Start mining
    nohup ./target/release/demle-miner \
        --contract "$CONTRACT_ADDRESS" \
        --rpc-url "http://$COORDINATOR_IP:8545" \
        --threads 8 \
        --target-teraflops 50.0 \
        --verbose > miner_$miner_id.log 2>&1 &
        
    echo "✅ H100 Miner #$miner_id started"
}

# Function to check GPU status
check_gpu() {
    echo "🔍 Checking GPU status..."
    nvidia-smi
    echo ""
    echo "💾 GPU Memory:"
    nvidia-smi --query-gpu=memory.total,memory.used,memory.free --format=csv,noheader,nounits
}

# Function to monitor mining
monitor_mining() {
    echo "📊 Starting mining monitor..."
    
    while true; do
        echo "$(date): Mining Status Report"
        echo "─────────────────────────────"
        
        # Check coordinator
        if curl -s "http://$COORDINATOR_IP:8545" > /dev/null; then
            echo "✅ Coordinator: Online"
        else
            echo "❌ Coordinator: Offline"
        fi
        
        # Check GPU utilization
        echo "🔥 GPU Utilization:"
        nvidia-smi --query-gpu=utilization.gpu --format=csv,noheader,nounits | while read util; do
            echo "   GPU: ${util}%"
        done
        
        # Check mining logs
        if [ -f "miner_1.log" ]; then
            TOKENS=$(tail -20 miner_1.log | grep "Tokens:" | tail -1 | awk '{print $5}')
            echo "🪙 Tokens Earned: $TOKENS"
        fi
        
        echo ""
        sleep 60
    done
}

# Main execution
case "${4:-setup}" in
    "coordinator")
        setup_coordinator
        ;;
    "miner")
        check_gpu
        setup_h100_miner 1
        ;;
    "monitor")
        monitor_mining
        ;;
    "setup")
        if [ -z "$CONTRACT_ADDRESS" ]; then
            echo "❌ Contract address required!"
            echo "Usage: $0 <coordinator_ip> <contract_address> [instance_count] [mode]"
            exit 1
        fi
        
        # Check if this is coordinator or miner instance
        if [ "$COORDINATOR_IP" = "localhost" ] || [ "$COORDINATOR_IP" = "127.0.0.1" ]; then
            setup_coordinator
        else
            setup_h100_miner 1
        fi
        ;;
    *)
        echo "Unknown mode: $4"
        echo "Available modes: coordinator, miner, monitor, setup"
        exit 1
        ;;
esac

echo "�� Setup complete!" 