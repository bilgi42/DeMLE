#!/bin/bash

# Quick setup script for DEMLE mining on H100
# Usage: curl -sSL <script-url> | bash -s -- CONTRACT_ADDRESS RPC_URL

set -e

CONTRACT_ADDRESS=$1
RPC_URL=$2

if [ -z "$CONTRACT_ADDRESS" ] || [ -z "$RPC_URL" ]; then
    echo "❌ Usage: $0 <contract_address> <rpc_url>"
    echo "Example: $0 0x1234... http://192.168.1.100:8545"
    exit 1
fi

echo "🚀 Setting up DEMLE H100 Miner"
echo "📍 Contract: $CONTRACT_ADDRESS"
echo "🌐 RPC: $RPC_URL"

# Update system
echo "📦 Installing dependencies..."
apt update && apt install -y curl build-essential git

# Install Rust
echo "🦀 Installing Rust..."
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source ~/.cargo/env

# Verify CUDA
echo "🔥 Checking CUDA..."
if ! command -v nvidia-smi &> /dev/null; then
    echo "❌ CUDA/GPU not found! Make sure you're using a GPU instance."
    exit 1
fi

nvidia-smi

# Clone and build
echo "🛠️  Building DEMLE miner..."
git clone https://github.com/bilgilovelace/DeMLE.git
cd DeMLE/demle-miner

# Build with CUDA support
CUDA_HOME=/usr/local/cuda cargo build --release --features cuda

# Test connection to RPC
echo "🌐 Testing RPC connection..."
if ! curl -s --connect-timeout 5 "$RPC_URL" > /dev/null; then
    echo "❌ Cannot connect to RPC at $RPC_URL"
    echo "   Make sure your blockchain is running and accessible"
    exit 1
fi

echo "✅ RPC connection successful!"

# Start mining
echo "⛏️  Starting DEMLE mining..."
echo "🎯 Target: 100 TeraFLOPS on H100"
echo "📊 Monitor with: watch -n 1 nvidia-smi"

./target/release/demle-miner \
    --contract "$CONTRACT_ADDRESS" \
    --rpc-url "$RPC_URL" \
    --threads 16 \
    --target-teraflops 100.0 \
    --verbose 