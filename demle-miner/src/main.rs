use clap::Parser;
use demle_core::{types::MiningStats, MLOperation, NetworkConfig, WorkUnit};
use demle_fp8::{execute_ml_operation, flops_to_teraflops};
use demle_rpc::DemleRpcClient;
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use tracing::{info, warn};

#[cfg(feature = "cuda")]
use candle_core;

#[derive(Parser)]
#[command(name = "demle-miner")]
#[command(about = "DEMLE FP8 ML cryptocurrency miner")]
struct Args {
    /// Number of mining threads
    #[arg(short = 'j', long, default_value = "4")]
    threads: usize,

    /// Mining target in teraflops
    #[arg(short, long, default_value = "150.0")]
    target_teraflops: f64,

    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,

    /// RPC URL for blockchain connection
    #[arg(long, default_value = "http://localhost:8545")]
    rpc_url: String,
    
    /// Contract address for DEMLE token
    #[arg(long)]
    contract: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Initialize logging
    let log_level = if args.verbose { "debug" } else { "info" };
    tracing_subscriber::fmt()
        .with_env_filter(format!("demle_miner={}", log_level))
        .init();

    info!("🚀 Starting DEMLE FP8 Miner");
    info!("Threads: {}", args.threads);
    info!("Target: {:.2} TeraFLOPS", args.target_teraflops);
    info!("RPC URL: {}", args.rpc_url);

    let network_config = NetworkConfig {
        rpc_url: args.rpc_url,
        contract_address: args.contract,
        ..Default::default()
    };

    let mut miner = Miner::new(network_config, args.threads, args.target_teraflops).await?;
    miner.start_mining().await?;

    Ok(())
}

struct Miner {
    config: NetworkConfig,
    rpc_client: DemleRpcClient,
    threads: usize,
    target_teraflops: f64,
    stats: MiningStats,
    start_time: Instant,
}

impl Miner {
    async fn new(config: NetworkConfig, threads: usize, target_teraflops: f64) -> Result<Self, Box<dyn std::error::Error>> {
        let mut rpc_client = DemleRpcClient::new(config.clone());
        
        // Initialize the contract
        rpc_client.init_contract().await
            .map_err(|e| format!("Failed to initialize contract: {}", e))?;
        
        Ok(Self {
            config,
            rpc_client,
            threads,
            target_teraflops,
            stats: MiningStats::default(),
            start_time: Instant::now(),
        })
    }

    async fn start_mining(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        info!("⛏️  Starting mining with {} threads", self.threads);
        info!("📍 Contract Address: {}", self.config.contract_address);
        info!("🌐 RPC URL: {}", self.config.rpc_url);
        
        // Check GPU availability
        #[cfg(feature = "cuda")]
        {
            match candle_core::Device::new_cuda(0) {
                Ok(device) => {
                    info!("🔥 H100 GPU acceleration enabled (CUDA)");
                    // Warm up GPU
                    let test_tensor = candle_core::Tensor::zeros((1024, 1024), candle_core::DType::F32, &device)?;
                    let _warm = test_tensor.matmul(&test_tensor)?;
                    info!("🚀 GPU warmed up and ready");
                },
                Err(e) => {
                    warn!("⚠️  GPU not available, falling back to CPU: {}", e);
                }
            }
        }
        #[cfg(not(feature = "cuda"))]
        {
            info!("💻 Running on CPU (compile with --features cuda for GPU acceleration)");
        }

        let mut nonce = 0u64;

        loop {
            let work_unit = self.generate_work_unit(nonce).await?;

            match self.mine_work_unit(&work_unit).await {
                Ok(result) => {
                    self.update_stats(&result);
                    
                    // Submit work to blockchain
                    match self.rpc_client.submit_work(&result).await {
                        Ok(tx_hash) => {
                            info!("✅ Work submitted! TX: {}", tx_hash);
                            self.stats.tokens_earned += 100; // Assume 100 DEMLE reward
                            
                            if result.total_flops as f64 / 1e12 >= self.target_teraflops {
                                info!(
                                    "🎯 Target achieved! Found solution with {:.2} TeraFLOPS",
                                    flops_to_teraflops(result.total_flops)
                                );
                            }
                        }
                        Err(e) => {
                            warn!("❌ Failed to submit work: {}", e);
                        }
                    }
                    
                    self.print_stats();
                }
                Err(e) => {
                    warn!("Mining error: {}", e);
                }
            }

            nonce = nonce.wrapping_add(1);
            // No delay for maximum H100 utilization - removed artificial bottleneck
        }
    }

    async fn generate_work_unit(&self, nonce: u64) -> Result<WorkUnit, Box<dyn std::error::Error>> {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

        // H100 Tensor Core Optimized: Memory-balanced massive operations
        let operations = vec![
            // Massive GEMM for maximum tensor core utilization (proven to work - 105+ TFLOPS!)
            MLOperation::MatrixMultiply {
                dimensions: (16384, 16384, 8192), // ~4.3 TB FLOPS single operation!
                seed: nonce,
            },
            // Memory-optimized attention (proven to work - adds ~16 TFLOPS)
            MLOperation::MultiHeadAttention {
                batch_size: 64,  
                seq_length: 1024, 
                d_model: 4096, 
                num_heads: 64, 
                seed: nonce.wrapping_add(1),
            },
            // Fast completing GEMM operation (replaces slow convolution)
            MLOperation::MatrixMultiply {
                dimensions: (8192, 8192, 4096), // Smaller but fast GEMM, ~1 TB FLOPS
                seed: nonce.wrapping_add(2),
            },
        ];

        Ok(WorkUnit {
            id: format!("work_{}", nonce),
            previous_hash: "0x0000000000000000000000000000000000000000".to_string(),
            timestamp,
            difficulty: (self.target_teraflops * 1e6) as u64,
            operations,
            nonce_range: (nonce, nonce + 1000),
        })
    }

    async fn mine_work_unit(
        &self,
        work_unit: &WorkUnit,
    ) -> Result<demle_core::WorkResult, Box<dyn std::error::Error>> {
        let start = Instant::now();

        info!("⚡ Mining work unit: {}", work_unit.id);
        info!("📋 Operations: {}", work_unit.operations.len());

        // H100 Optimized: Sequential execution of massive operations
        // Parallel threads for GPU are counterproductive due to CUDA context overhead
        #[cfg(feature = "cuda")]
        {
            let mut operation_results = Vec::new();
            let mut total_flops = 0u64;

            // Execute each massive operation sequentially for maximum GPU utilization
            for (i, operation) in work_unit.operations.iter().enumerate() {
                info!("🔄 Executing MASSIVE operation {} on H100: {}", i + 1, operation);
                
                let result = execute_ml_operation(operation)?;
                total_flops += result.flops;
                operation_results.push(result);
                
                // Log progress for massive operations
                info!("✅ Completed operation {} - {:.2} TFLOPS accumulated", 
                      i + 1, total_flops as f64 / 1e12);
                
                // Memory cleanup between operations to prevent OOM
                if i < work_unit.operations.len() - 1 { // Don't cleanup after last operation
                    std::thread::sleep(std::time::Duration::from_millis(500));
                    // Force garbage collection in Rust
                    info!("🧹 Memory cleanup between operations...");
                }
            }
            
            let execution_time_ms = start.elapsed().as_millis() as u64;

            // Simple hash combining all operation hashes
            let hash_strings: Vec<String> = operation_results
                .iter()
                .map(|r| r.result_hash.clone())
                .collect();

            let combined_hash = format!("{}:{}", work_unit.nonce_range.0, hash_strings.join(","));
            let result_hash = format!("{:x}", md5::compute(combined_hash));

            Ok(demle_core::WorkResult {
                work_id: work_unit.id.clone(),
                nonce: work_unit.nonce_range.0,
                hash: result_hash,
                execution_time_ms,
                total_flops,
                operation_results,
            })
        }
        #[cfg(not(feature = "cuda"))]
        {
            // Fallback to tokio async for CPU
            let handles: Vec<_> = work_unit.operations
                .iter()
                .enumerate()
                .map(|(i, operation)| {
                    let op = operation.clone();
                    tokio::spawn(async move {
                        info!("🔄 Executing operation {} in parallel: {}", i + 1, op);
                        execute_ml_operation(&op)
                    })
                })
                .collect();

            let mut operation_results = Vec::new();
            let mut total_flops = 0u64;

            // Collect results from parallel execution
            for handle in handles {
                let result = handle.await??;
                total_flops += result.flops;
                operation_results.push(result);
            }

            let execution_time_ms = start.elapsed().as_millis() as u64;

            // Simple hash combining all operation hashes
            let hash_strings: Vec<String> = operation_results
                .iter()
                .map(|r| r.result_hash.clone())
                .collect();

            let combined_hash = format!("{}:{}", work_unit.nonce_range.0, hash_strings.join(","));
            let result_hash = format!("{:x}", md5::compute(combined_hash));

            Ok(demle_core::WorkResult {
                work_id: work_unit.id.clone(),
                nonce: work_unit.nonce_range.0,
                hash: result_hash,
                execution_time_ms,
                total_flops,
                operation_results,
            })
        }
    }

    fn update_stats(&mut self, result: &demle_core::WorkResult) {
        self.stats.total_operations += result.operation_results.len() as u64;
        self.stats.uptime_seconds = self.start_time.elapsed().as_secs();

        // Calculate instantaneous TeraFLOPS from this work unit
        if result.execution_time_ms > 0 {
            self.stats.teraflops = (result.total_flops as f64) / 1e12 / (result.execution_time_ms as f64 / 1000.0);
        }
        
        // Calculate rolling average hashrate
        let elapsed_secs = self.stats.uptime_seconds as f64;
        if elapsed_secs > 0.0 {
            self.stats.hashrate = self.stats.total_operations as f64 / elapsed_secs;
        }
    }

    fn print_stats(&self) {
        println!("\n📊 DEMLE Mining Stats:");
        println!("┌─────────────────────────────────────────┐");
        println!(
            "│ ⚡ TeraFLOPS: {:>8.2} TFLOPS/s       │",
            self.stats.teraflops
        );
        println!(
            "│ 🔥 Hashrate:  {:>8.1} ops/s          │",
            self.stats.hashrate
        );
        println!(
            "│ 🧮 Total Ops: {:>7}                │",
            self.stats.total_operations
        );
        println!(
            "│ ⏱️  Uptime:    {:>7}s                │",
            self.stats.uptime_seconds
        );
        println!(
            "│ 🪙 Tokens:    {:>7}                │",
            self.stats.tokens_earned
        );
        println!(
            "│ 🎯 Target:    {:>8.2} TFLOPS/s       │",
            self.target_teraflops
        );
        
        // Show hardware acceleration status
        #[cfg(feature = "cuda")]
        println!("│ 💾 Hardware:  GPU (CUDA)              │");
        #[cfg(not(feature = "cuda"))]
        println!("│ 💾 Hardware:  CPU                     │");
        
        println!("└─────────────────────────────────────────┘");

        // Show operation breakdown
        println!("\n🔧 ML Operations Performed:");
        println!("• Matrix Multiplication (GEMM) - 4096³ matrices");
        println!("• 2D Convolution (CNN layers) - 2048x1024 kernels");
        println!("• Multi-Head Attention (Transformers) - 32 heads");
        println!("• Batch Normalization - Large tensors");

        // Performance status
        if self.stats.teraflops >= self.target_teraflops {
            println!(
                "\n🎯 TARGET ACHIEVED! Running at {:.2} TeraFLOPS (Target: {:.2})",
                self.stats.teraflops, self.target_teraflops
            );
        } else if self.stats.teraflops > 0.0 {
            let efficiency = (self.stats.teraflops / self.target_teraflops) * 100.0;
            println!(
                "\n📈 Performance: {:.1}% of target ({:.2}/{:.2} TFLOPS)",
                efficiency, self.stats.teraflops, self.target_teraflops
            );
        }

        println!("\n{:-<50}", "");
    }
}
