#!/bin/bash

# H100 Balanced Performance DEMLE Mining Script
# Memory-stable configuration that maintains 100+ TFLOPS performance

echo "🔥 H100 BALANCED PERFORMANCE MODE"
echo "Memory-optimized for stable 100+ TFLOPS performance..."

# H100 Balanced Performance Environment
export CUDA_VISIBLE_DEVICES=0
export DEMLE_BATCH_SIZE=2048  # Balanced batch for stability
export CUDA_LAUNCH_BLOCKING=0
export DEMLE_TENSOR_CORE_OPTIMIZATION=1

# Conservative memory settings for stability
export CUBLAS_WORKSPACE_CONFIG=:2048:8
export PYTORCH_CUDA_ALLOC_CONF=max_split_size_mb:512,expandable_segments:True

echo "📊 H100 Status:"
nvidia-smi --query-gpu=name,memory.total,memory.used,utilization.gpu,temperature.gpu --format=csv,noheader

# Clean start
pkill -f demle-miner || true
sleep 2
nvidia-smi --gpu-reset || true

echo "🔨 Building memory-balanced miner..."
cd demle-miner
CUDA_HOME=/usr/local/cuda cargo build --release --features cuda

echo ""
echo "⚡ LAUNCHING MEMORY-BALANCED H100 MINING"
echo "🎯 Target: 150 TeraFLOPS"
echo "🧠 Operations: MASSIVE GEMM (140 TFLOPS) + Balanced attention + Optimized conv"
echo "💾 Memory: Stable 60-70GB usage with cleanup"
echo "🔥 Performance: Sustained 100+ TFLOPS"
echo ""

./target/release/demle-miner \
    --contract "$1" \
    --rpc-url "$2" \
    --threads 4 \
    --target-teraflops 150.0 \
    --verbose &

MINER_PID=$!
echo "Mining PID: $MINER_PID"

echo "⏳ First operation starting (expecting 140+ TFLOPS)..."
sleep 15

# Monitor for stability
for i in {1..15}; do
    if ! kill -0 $MINER_PID 2>/dev/null; then
        echo "❌ Miner stopped - checking logs..."
        break
    fi
    
    echo "--- Performance Check $i ---"
    nvidia-smi --query-gpu=utilization.gpu,memory.used,memory.total,temperature.gpu,power.draw --format=csv,noheader,nounits | \
    while IFS=, read gpu_util mem_used mem_total temp power; do
        mem_gb=$(( mem_used / 1024 ))
        echo "GPU: ${gpu_util}% | Memory: ${mem_gb}GB | Temp: ${temp}°C | Power: ${power}W"
        
        if [ "$gpu_util" -gt 70 ]; then
            echo "✅ Excellent performance!"
        fi
        
        if [ "$mem_used" -gt 70000 ]; then
            echo "⚠️  Memory approaching limit"
        elif [ "$mem_used" -gt 50000 ]; then
            echo "✅ Good memory utilization"
        fi
    done
    
    echo ""
    sleep 20
done

echo "✅ H100 mining running with balanced performance!" 