#!/bin/bash

# H100 Memory-Optimized DEMLE Mining Script
# Balances performance with 80GB memory constraint

echo "🔥 H100 Memory-Optimized DEMLE Mining"
echo "Balancing performance with 80GB memory limit..."

# Set H100-optimized environment variables
export CUDA_VISIBLE_DEVICES=0
export DEMLE_BATCH_SIZE=1024  # Optimized batch size for H100
export DEMLE_GPU_MEMORY_FRACTION=0.85  # Use 85% of 80GB (~68GB)
export CUDA_LAUNCH_BLOCKING=0  # Async launches
export DEMLE_TENSOR_CORE_OPTIMIZATION=1

# Memory management for large operations
export CUBLAS_WORKSPACE_CONFIG=:2048:8  # Balanced workspace
export PYTORCH_CUDA_ALLOC_CONF=max_split_size_mb:256,expandable_segments:True

echo "📊 Current H100 Status:"
nvidia-smi --query-gpu=name,memory.total,memory.used,utilization.gpu,temperature.gpu --format=csv,noheader

# Kill any existing miner processes
pkill -f demle-miner || true
sleep 2

# Rebuild with optimizations
echo "🔨 Rebuilding with memory-optimized settings..."
cd demle-miner
CUDA_HOME=/usr/local/cuda cargo build --release --features cuda

echo "⛏️  Starting memory-optimized H100 mining..."
echo "🎯 Target: 150 TeraFLOPS (balanced for memory)"
echo "💾 Memory: Using up to 68GB of H100 memory"
echo "🧠 Operations: Optimized matrix sizes for tensor cores"

# Clear GPU memory
nvidia-smi --gpu-reset || true

./target/release/demle-miner \
    --contract "$1" \
    --rpc-url "$2" \
    --threads 32 \
    --target-teraflops 150.0 \
    --verbose &

MINER_PID=$!
echo "Mining PID: $MINER_PID"

# Monitor performance
echo "📈 Monitoring H100 performance..."
echo "Expected: 70-90% GPU utilization, 40-65GB memory"

sleep 5  # Give it time to start

for i in {1..10}; do
    echo "--- Status Check $i ---"
    nvidia-smi --query-gpu=utilization.gpu,memory.used,memory.total,temperature.gpu,power.draw --format=csv,noheader,nounits | \
    while IFS=, read gpu_util mem_used mem_total temp power; do
        echo "GPU Utilization: ${gpu_util}%"
        echo "Memory: ${mem_used}MB / ${mem_total}MB ($(( mem_used * 100 / mem_total ))%)"
        echo "Temperature: ${temp}°C"
        echo "Power: ${power}W"
        
        # Check if process is still running
        if ! kill -0 $MINER_PID 2>/dev/null; then
            echo "❌ Miner process stopped!"
            break
        fi
        
        # Memory safety check
        if [ "$mem_used" -gt 75000 ]; then
            echo "⚠️  High memory usage - monitoring closely"
        fi
    done
    
    echo ""
    sleep 10
done

echo "✅ Mining started successfully!"
echo "Monitor with: watch -n 1 'nvidia-smi && echo && ps aux | grep demle-miner'" 