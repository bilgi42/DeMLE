# 🚀 DEMLE H100 Mining on Hyperbolic

Complete guide for deploying DEMLE mining on Hyperbolic's H100 GPU cloud.

## 📋 Prerequisites

1. **Hyperbolic Account**: Sign up at [hyperbolic.xyz](https://hyperbolic.xyz)
2. **SSH Key**: Upload your public key to Hyperbolic
3. **Git Repository**: Push your DEMLE code to GitHub (make it public or add deploy keys)

## 🏗️ Architecture Overview

```
Internet
    │
    ├─ Coordinator Instance (CPU, 4GB RAM)
    │  ├─ Hardhat Blockchain Node
    │  ├─ DEMLE Smart Contract  
    │  └─ Real-time Dashboard
    │
    └─ H100 Mining Fleet
       ├─ H100 Instance #1 ──┐
       ├─ H100 Instance #2   ├─► Mining Pool
       ├─ H100 Instance #3   │   (GPU Accelerated)
       └─ H100 Instance #4 ──┘
```

## 🚀 Step-by-Step Deployment

### Step 1: Launch Coordinator Instance

```bash
# 1. Create coordinator instance on Hyperbolic
# - Instance Type: CPU (t3.medium equivalent)
# - RAM: 4GB
# - Storage: 20GB
# - OS: Ubuntu 22.04

# 2. SSH into coordinator
ssh ubuntu@<coordinator-ip>

# 3. Run coordinator setup
curl -fsSL https://raw.githubusercontent.com/your-username/DeMLE/main/deploy/hyperbolic-setup.sh | bash -s -- localhost "" 1 coordinator

# 4. Get the contract address
cat /tmp/contract_address
```

### Step 2: Launch H100 Mining Instances

```bash
# For each H100 instance you want to launch:

# 1. Create H100 instance on Hyperbolic
# - Instance Type: H100 (80GB HBM3)
# - RAM: 32GB+
# - Storage: 50GB
# - OS: Ubuntu 22.04 with CUDA

# 2. SSH into H100 instance
ssh ubuntu@<h100-instance-ip>

# 3. Run miner setup (replace with your coordinator IP and contract address)
curl -fsSL https://raw.githubusercontent.com/your-username/DeMLE/main/deploy/hyperbolic-setup.sh | bash -s -- <coordinator-ip> <contract-address> 1 miner
```

### Step 3: Monitor Your Mining Fleet

```bash
# On coordinator instance, start monitoring
./hyperbolic-setup.sh <coordinator-ip> <contract-address> 1 monitor

# Or view dashboard in browser
# Navigate to: http://<coordinator-ip>:3000
```

## ⚙️ Configuration Optimization

### H100 Tuning Parameters

```bash
# For maximum H100 utilization, tune these parameters:

# 1. Increase batch sizes for large GPU memory
export DEMLE_BATCH_SIZE=512

# 2. Use multiple GPU streams
export DEMLE_GPU_STREAMS=4

# 3. Optimize for H100 tensor cores
export DEMLE_USE_TENSOR_CORES=true

# 4. Set optimal thread count (H100 has massive parallelism)
--threads 16 --target-teraflops 100.0
```

### Cost Optimization

```bash
# H100 instances are expensive (~$4-8/hour)
# Optimize costs by:

# 1. Use spot instances when available
# 2. Scale fleet based on demand
# 3. Monitor GPU utilization (aim for >90%)
# 4. Auto-shutdown on low profitability
```

## 📊 Expected Performance

| Instance Type | TFLOPS/s | Tokens/Hour | Cost/Hour | ROI |
|---------------|----------|-------------|-----------|-----|
| H100 80GB     | 50-100   | ~3,600      | $6.00     | TBD |
| 4x H100 Fleet | 200-400  | ~14,400     | $24.00    | TBD |

## 🔍 Monitoring & Debugging

### Real-time Monitoring

```bash
# GPU utilization
watch -n 1 nvidia-smi

# Mining logs
tail -f miner_1.log

# Network connectivity
ping <coordinator-ip>

# Token balance
curl -X POST -H "Content-Type: application/json" \
  --data '{"jsonrpc":"2.0","method":"eth_getBalance","params":["<your-address>","latest"],"id":1}' \
  http://<coordinator-ip>:8545
```

### Common Issues

1. **CUDA Not Found**
   ```bash
   # Install CUDA toolkit
   sudo apt update
   sudo apt install nvidia-cuda-toolkit
   export CUDA_HOME=/usr/local/cuda
   ```

2. **Network Timeout**
   ```bash
   # Check firewall rules
   sudo ufw status
   sudo ufw allow 8545/tcp
   ```

3. **Memory Issues**
   ```bash
   # Monitor GPU memory
   watch -n 1 "nvidia-smi --query-gpu=memory.used,memory.total --format=csv"
   ```

## 💰 Economic Considerations

### Revenue Calculation
```
Daily Tokens = Mining_Rate × 24 hours
Daily Revenue = Daily_Tokens × Token_Price
Daily Costs = H100_Cost_Per_Hour × 24 × Instance_Count
Daily Profit = Daily_Revenue - Daily_Costs
```

### Break-even Analysis
```bash
# Example for 4x H100 setup:
# Cost: $24/hour × 24 = $576/day
# Tokens: ~14,400 DEMLE/day
# Break-even: Token needs to be worth $0.04+ each
```

## 🛡️ Security Best Practices

1. **Network Security**
   ```bash
   # Use VPN or private networks
   # Restrict RPC access to mining fleet only
   # Use SSL/TLS for all connections
   ```

2. **Key Management**
   ```bash
   # Use hardware security modules for private keys
   # Rotate credentials regularly
   # Monitor for unauthorized access
   ```

## 📈 Scaling Strategies

### Horizontal Scaling
```bash
# Add more H100 instances as needed
# Use load balancer for coordinator
# Implement auto-scaling based on profitability
```

### Vertical Scaling
```bash
# Upgrade to H100 SXM (faster interconnect)
# Use multi-GPU instances (8x H100)
# Optimize memory bandwidth utilization
```

## 🔧 Advanced Features

### Multi-Region Deployment
```bash
# Deploy coordinator in low-latency region
# Distribute miners across regions for redundancy
# Use CDN for dashboard access
```

### Automated Management
```bash
# Use Hyperbolic's API for programmatic instance management
# Implement auto-restart on failures
# Dynamic pricing optimization
```

---

## 🆘 Support

- **Hyperbolic Docs**: [hyperbolic.xyz/docs](https://hyperbolic.xyz/docs)
- **DEMLE Community**: [Discord/Telegram links]
- **Issues**: Create GitHub issues for bugs

---

**💡 Pro Tips:**
- Start with 1-2 H100s to test setup
- Monitor GPU utilization closely (aim for >90%)
- Consider using spot instances for cost savings
- Set up alerting for downtime or performance issues 