use clap::Parser;
use demle_core::{MLOperation, NetworkConfig, WorkUnit, types::MiningStats};
use demle_fp8::{execute_ml_operation, calculate_total_flops, flops_to_teraflops};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tokio::time::sleep;
use tracing::{info, warn, error};

#[derive(Parser)]
#[command(name = "demle-miner")]
#[command(about = "DEMLE FP8 ML cryptocurrency miner")]
struct Args {
    /// Number of mining threads
    #[arg(short = 'j', long, default_value = "4")]
    threads: usize,
    
    /// Mining target in teraflops
    #[arg(short, long, default_value = "1.0")]
    target_teraflops: f64,
    
    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,
    
    /// RPC URL for blockchain connection
    #[arg(long, default_value = "http://localhost:8545")]
    rpc_url: String,
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
        ..Default::default()
    };
    
    let mut miner = Miner::new(network_config, args.threads, args.target_teraflops);
    miner.start_mining().await?;
    
    Ok(())
}

struct Miner {
    config: NetworkConfig,
    threads: usize,
    target_teraflops: f64,
    stats: MiningStats,
    start_time: Instant,
}

impl Miner {
    fn new(config: NetworkConfig, threads: usize, target_teraflops: f64) -> Self {
        Self {
            config,
            threads,
            target_teraflops,
            stats: MiningStats::default(),
            start_time: Instant::now(),
        }
    }
    
    async fn start_mining(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        info!("⛏️  Starting mining with {} threads", self.threads);
        
        let mut nonce = 0u64;
        
        loop {
            let work_unit = self.generate_work_unit(nonce).await?;
            
            match self.mine_work_unit(&work_unit).await {
                Ok(result) => {
                    self.update_stats(&result);
                    self.print_stats();
                    
                    if result.total_flops as f64 / 1e12 >= self.target_teraflops {
                        info!("🎯 Target achieved! Found solution with {:.2} TeraFLOPS", 
                              flops_to_teraflops(result.total_flops));
                    }
                }
                Err(e) => {
                    warn!("Mining error: {}", e);
                }
            }
            
            nonce = nonce.wrapping_add(1);
            sleep(Duration::from_millis(100)).await;
        }
    }
    
    async fn generate_work_unit(&self, nonce: u64) -> Result<WorkUnit, Box<dyn std::error::Error>> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_secs();
        
        // Generate a mix of ML operations for mining
        let operations = vec![
            MLOperation::MatrixMultiply {
                dimensions: (256, 256, 256),
                seed: nonce,
            },
            MLOperation::Convolution2D {
                input_shape: (8, 64, 16, 16),
                kernel_shape: (128, 64, 3, 3),
                stride: (1, 1),
                padding: (1, 1),
                seed: nonce.wrapping_add(1),
            },
            MLOperation::MultiHeadAttention {
                batch_size: 4,
                seq_length: 32,
                d_model: 128,
                num_heads: 8,
                seed: nonce.wrapping_add(2),
            },
            MLOperation::BatchNormalization {
                shape: (8, 128, 16, 16),
                epsilon: 1e-5,
                seed: nonce.wrapping_add(3),
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
    
    async fn mine_work_unit(&self, work_unit: &WorkUnit) -> Result<demle_core::WorkResult, Box<dyn std::error::Error>> {
        let start = Instant::now();
        
        info!("⚡ Mining work unit: {}", work_unit.id);
        info!("📋 Operations: {}", work_unit.operations.len());
        
        let mut operation_results = Vec::new();
        let mut total_flops = 0u64;
        
        for (i, operation) in work_unit.operations.iter().enumerate() {
            info!("🔄 Executing operation {}: {}", i + 1, operation);
            
            let result = execute_ml_operation(operation)?;
            total_flops += result.flops;
            operation_results.push(result);
        }
        
        let execution_time_ms = start.elapsed().as_millis() as u64;
        
        // Simple hash combining all operation hashes
        let hash_strings: Vec<String> = operation_results.iter()
            .map(|r| r.result_hash.clone())
            .collect();
        
        let combined_hash = format!("{}:{}", 
            work_unit.nonce_range.0,
            hash_strings.join(",")
        );
        
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
    
    fn update_stats(&mut self, result: &demle_core::WorkResult) {
        self.stats.total_operations += result.operation_results.len() as u64;
        self.stats.uptime_seconds = self.start_time.elapsed().as_secs();
        
        // Calculate rolling averages
        let elapsed_secs = self.stats.uptime_seconds as f64;
        if elapsed_secs > 0.0 {
            self.stats.teraflops = (result.total_flops as f64) / 1e12 / (result.execution_time_ms as f64 / 1000.0);
            self.stats.hashrate = self.stats.total_operations as f64 / elapsed_secs;
        }
    }
    
    fn print_stats(&self) {
        println!("\n📊 DEMLE Mining Stats:");
        println!("┌─────────────────────────────────────────┐");
        println!("│ ⚡ TeraFLOPS: {:>8.2} TFLOPS/s       │", self.stats.teraflops);
        println!("│ 🔥 Hashrate:  {:>8.1} ops/s          │", self.stats.hashrate);
        println!("│ 🧮 Total Ops: {:>7}                │", self.stats.total_operations);
        println!("│ ⏱️  Uptime:    {:>7}s                │", self.stats.uptime_seconds);
        println!("│ 🪙 Tokens:    {:>7}                │", self.stats.tokens_earned);
        println!("└─────────────────────────────────────────┘");
        
        // Show operation breakdown
        println!("\n🔧 ML Operations Performed:");
        println!("• Matrix Multiplication (GEMM)");
        println!("• 2D Convolution (CNN layers)");
        println!("• Multi-Head Attention (Transformers)");
        println!("• Batch Normalization");
        
        if self.stats.teraflops >= self.target_teraflops {
            println!("\n🎯 TARGET ACHIEVED! Running at {:.2} TeraFLOPS", self.stats.teraflops);
        }
        
        println!("\n{:-<50}", "");
    }
} 