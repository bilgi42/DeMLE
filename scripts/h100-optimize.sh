#!/bin/bash

# H100 Optimization Script for DEMLE Mining
# Maximizes H100 80GB utilization

echo "🔥 H100 DEMLE Mining Optimization"
echo "Configuring for maximum tensor core utilization..."

# Set H100-specific environment variables
export CUDA_VISIBLE_DEVICES=0
export DEMLE_BATCH_SIZE=2048  # Massive batch size for H100
export DEMLE_GPU_MEMORY_FRACTION=0.95  # Use 95% of 80GB
export CUDA_LAUNCH_BLOCKING=0  # Async launches
export DEMLE_TENSOR_CORE_OPTIMIZATION=1

# H100 tensor core optimization
export CUBLAS_WORKSPACE_CONFIG=:4096:8  # More workspace for larger ops
export PYTORCH_CUDA_ALLOC_CONF=max_split_size_mb:512

# Check current GPU status
echo "📊 Current H100 Status:"
nvidia-smi --query-gpu=name,memory.total,memory.used,utilization.gpu,utilization.memory,temperature.gpu --format=csv,noheader

# Rebuild miner with H100 optimizations
echo "🔨 Rebuilding miner with H100 optimizations..."
cd demle-miner
CUDA_HOME=/usr/local/cuda cargo build --release --features cuda

# Run with H100-optimized settings
echo "⛏️  Starting H100-optimized mining..."
echo "🎯 Target: 200+ TeraFLOPS"
echo "💾 Memory: Using up to 76GB of H100 memory"
echo "🔥 Tensor Cores: Enabled with BF16 precision"

./target/release/demle-miner \
    --contract "$1" \
    --rpc-url "$2" \
    --threads 64 \
    --target-teraflops 200.0 \
    --verbose &

MINER_PID=$!

# Monitor H100 utilization
echo "📈 Monitoring H100 utilization (Ctrl+C to stop)..."
echo "Expected: 90%+ GPU utilization, 60GB+ memory usage"

while kill -0 $MINER_PID 2>/dev/null; do
    clear
    echo "🔥 H100 Real-time Status:"
    nvidia-smi --query-gpu=utilization.gpu,utilization.memory,memory.used,memory.total,temperature.gpu,power.draw --format=csv,noheader,nounits | \
    while IFS=, read gpu_util mem_util mem_used mem_total temp power; do
        echo "GPU Utilization: ${gpu_util}%"
        echo "Memory Utilization: ${mem_util}%"
        echo "Memory Used: ${mem_used}MB / ${mem_total}MB"
        echo "Temperature: ${temp}°C"
        echo "Power Draw: ${power}W"
        
        # Alert if underutilized
        if [ "$gpu_util" -lt 80 ]; then
            echo "⚠️  GPU utilization below 80% - check configuration"
        fi
        if [ "$mem_used" -lt 50000 ]; then
            echo "⚠️  Memory usage below 50GB - operations may be too small"
        fi
    done
    
    echo ""
    echo "Expected optimal values:"
    echo "- GPU Utilization: 85-95%"
    echo "- Memory Usage: 50-75GB"
    echo "- Temperature: 60-85°C"
    echo "- TFLOPS: 150-300+"
    
    sleep 2
done 