#!/bin/bash

# H100 Maximum Performance DEMLE Mining Script
# Designed for 3 massive operations that fully saturate H100 tensor cores

echo "🔥 H100 MAXIMUM PERFORMANCE MODE"
echo "Executing 3 MASSIVE operations per work unit..."

# H100 Maximum Performance Environment
export CUDA_VISIBLE_DEVICES=0
export DEMLE_BATCH_SIZE=4096  # Maximum batch for massive operations
export CUDA_LAUNCH_BLOCKING=0  # Async for maximum throughput
export DEMLE_TENSOR_CORE_OPTIMIZATION=1

# Maximum CUDA performance settings
export CUBLAS_WORKSPACE_CONFIG=:4096:16  # Maximum workspace
export CUDA_DEVICE_MAX_CONNECTIONS=32   # Maximum streams
export PYTORCH_CUDA_ALLOC_CONF=max_split_size_mb:1024,expandable_segments:True

echo "📊 H100 Pre-Launch Status:"
nvidia-smi --query-gpu=name,memory.total,memory.used,utilization.gpu,temperature.gpu,power.draw --format=csv,noheader

# Kill existing miners
pkill -f demle-miner || true
sleep 3

# Clear and reset GPU
nvidia-smi --gpu-reset || true
sleep 2

echo "🔨 Building MAXIMUM PERFORMANCE miner..."
cd demle-miner
CUDA_HOME=/usr/local/cuda cargo build --release --features cuda

echo ""
echo "⛏️  LAUNCHING H100 MAXIMUM PERFORMANCE MINING"
echo "🎯 Target: 150 TeraFLOPS"
echo "🧠 Strategy: 3 MASSIVE operations (16K×16K×8K GEMM, 128-head attention, 256-batch conv)"
echo "💾 Memory: Will use 70-80GB of H100 memory"
echo "🔥 Tensor Cores: Maximum BF16 utilization"
echo ""

# Launch with minimal threads (GPU doesn't need many)
./target/release/demle-miner \
    --contract "$1" \
    --rpc-url "$2" \
    --threads 4 \
    --target-teraflops 150.0 \
    --verbose &

MINER_PID=$!
echo "Mining PID: $MINER_PID"

# Give it time to start the first massive operation
echo "⏳ Starting massive operations (this will take 30-60 seconds)..."
sleep 10

# Real-time monitoring
echo "📈 Real-time H100 monitoring:"
echo "Expected after first operation: 80%+ GPU utilization, 60GB+ memory"

for i in {1..20}; do
    if ! kill -0 $MINER_PID 2>/dev/null; then
        echo "❌ Miner stopped! Check logs above."
        exit 1
    fi
    
    echo "--- Check $i ($(date +%H:%M:%S)) ---"
    nvidia-smi --query-gpu=utilization.gpu,memory.used,memory.total,temperature.gpu,power.draw --format=csv,noheader,nounits | \
    while IFS=, read gpu_util mem_used mem_total temp power; do
        mem_percent=$(( mem_used * 100 / mem_total ))
        echo "GPU: ${gpu_util}% | Memory: ${mem_used}MB (${mem_percent}%) | Temp: ${temp}°C | Power: ${power}W"
        
        # Performance indicators
        if [ "$gpu_util" -gt 80 ]; then
            echo "✅ Excellent GPU utilization!"
        elif [ "$gpu_util" -gt 50 ]; then
            echo "🟡 Good GPU utilization"
        elif [ "$gpu_util" -lt 10 ]; then
            echo "⚠️  Low GPU utilization - operation may be CPU bound"
        fi
        
        if [ "$mem_used" -gt 60000 ]; then
            echo "✅ High memory usage - massive operations running"
        elif [ "$mem_used" -lt 10000 ]; then
            echo "⚠️  Low memory usage - operations may not be large enough"
        fi
        
        if [ "$power" -gt 400 ]; then
            echo "🔥 High power draw - H100 working hard!"
        fi
    done
    
    echo ""
    sleep 15
done

echo "✅ Mining successfully running with massive operations!"
echo ""
echo "🔍 To continue monitoring:"
echo "watch -n 5 'nvidia-smi && echo && tail -10 /tmp/miner.log'" 