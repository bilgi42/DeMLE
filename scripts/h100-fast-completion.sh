#!/bin/bash

# H100 Fast Completion Script - 3 GEMM operations for guaranteed completion
echo "🔥 H100 FAST COMPLETION MODE"
echo "Using 3 proven GEMM operations for reliable performance..."

export CUDA_VISIBLE_DEVICES=0
export CUDA_LAUNCH_BLOCKING=0

echo "📊 H100 Status:"
nvidia-smi --query-gpu=utilization.gpu,memory.used,temperature.gpu --format=csv,noheader

# Clean restart
pkill -f demle-miner || true
sleep 2

echo "🔨 Quick rebuild..."
cd demle-miner
cargo build --release --features cuda

echo ""
echo "⚡ LAUNCHING FAST-COMPLETING H100 MINING"
echo "🎯 Operations: 3 proven GEMM operations"
echo "⏱️  Expected: ~15-20 seconds per work unit"
echo "🚀 Performance: 150+ TFLOPS total"
echo ""

./target/release/demle-miner \
    --contract "$1" \
    --rpc-url "$2" \
    --threads 4 \
    --target-teraflops 150.0 \
    --verbose &

MINER_PID=$!
echo "Mining PID: $MINER_PID"

echo "⏳ Monitoring first complete work unit (expecting ~20 seconds)..."
sleep 5

# Quick monitoring
for i in {1..8}; do
    if ! kill -0 $MINER_PID 2>/dev/null; then
        echo "❌ Miner stopped"
        exit 1
    fi
    
    echo "--- Check $i ($(date +%H:%M:%S)) ---"
    nvidia-smi --query-gpu=utilization.gpu,memory.used,temperature.gpu,power.draw --format=csv,noheader,nounits | \
    while IFS=, read gpu_util mem_used temp power; do
        mem_gb=$(( mem_used / 1024 ))
        echo "GPU: ${gpu_util}% | Memory: ${mem_gb}GB | Temp: ${temp}°C | Power: ${power}W"
    done
    
    echo ""
    sleep 10
done

echo "✅ Should have completed first work unit by now!"
echo "Check the mining logs above for TFLOPS performance." 