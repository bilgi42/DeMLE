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

# Start demo environment
docker-compose up -d

# Run miner
cargo run --bin demle-miner --release
```

### Development
```bash
# Run tests
cargo test

# Run benchmarks
cargo bench

# Format code
cargo fmt && cargo clippy
```

## Architecture

```
Rust Miner (FP8 Operations) ↔ Smart Contract (Proof Verification) ↔ Blockchain (ERC-20 Token)
```

**Target**: Demonstrate productive mining that contributes to AI compute instead of wasting energy on arbitrary calculations.
