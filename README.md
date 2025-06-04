# DEMLE
**Decentralized Machine Learning Efforts on Blockchain**

> A proof-of-work cryptocurrency that replaces traditional SHA-256 mining with productive FP8 machine learning computations.

## What it does

Instead of wasting computational power on arbitrary hash calculations, DEMLE miners perform useful ML operations:
- Matrix multiplications (GEMM)
- 2D convolutions 
- Multi-head attention
- Batch normalization

All operations use FP8 precision (8-bit floating point) to align with modern AI accelerators like H100.

## Build & Run

### Prerequisites
- Rust 1.70+
- Node.js 18+
- Docker & Docker Compose

### Quick Start
```bash
# Build Rust components
cargo build --release

# Setup smart contracts
cd contracts && npm install && cd ..

# 🆕 Start live token distribution dashboard
cd contracts && npm run realtime-demo
# Open realtime-dashboard.html in your browser

# Start demo environment
docker-compose up -d

# Run miner (use contract address from dashboard)
cargo run --bin demle-miner --release -- --contract CONTRACT_ADDRESS --rpc http://localhost:8545
```

### 🔥 Real-time Token Dashboard

See your DEMLE tokens accumulate in real-time as you mine!

```bash
# Start the live dashboard
cd contracts && npm run realtime-demo

# Watch your miner appear and earn tokens live
# Dashboard updates every 3 seconds automatically
```

**Features:**
- 📊 Live token balance updates
- 👥 Multi-miner support  
- ⚡ Real-time mining events
- 📈 Dynamic charts and statistics
- 🎨 Visual animations when mining succeeds

### Development
```bash
# Run tests
cargo test

# Run benchmarks
cargo bench

# Format code
cargo fmt && cargo clippy

# Test smart contracts
cd contracts && npm test
```

## Architecture

```
Rust Miner (FP8 Operations) ↔ Smart Contract (Proof Verification) ↔ Blockchain (ERC-20 Token)
                                           ↓
                                Live Dashboard (Real-time Updates)
```

**Target**: Demonstrate productive mining that contributes to AI compute instead of wasting energy on arbitrary calculations.

## Considerations

This is just a showcase for a potential system architecture. There's no kind of optimizations, pytorch or any kind of an actual widely used ML library support at the time.
That being said, there's a bigger challenge in this system, verifying the proof
Probabilistic Sampling and reputation based proof of work systems can be implemented in the future
And maybe for faster execution speeds, there might be standalone primitive blockchain written in Rust that is just used for this project, but I didn't do that since project participation required the project to be in Ethereum network

## FAQ

>Isn't this slower than just spinning up a GPU Cluster?

Yes, it is. The main feature itself is not the execution speed, but the decentralization aspect. Governments are thinking of restricting processing power access to the countries they don't like, whether they can afford it or not. What we're trying to achieve is distribution of computing process and a reward mechanism attached to it

>Aren't there any other compaines trying to implement decentralised computing?

Again, yes, there is. One of them, Prime Intellect, is one of my favorite companies in 2025. But that being said, they're subject to US law, that comes from being a company, a centralized entity. One of the biggest adcantages of being deployed in a blockchain is that it's nearly untouchable from any entity.

