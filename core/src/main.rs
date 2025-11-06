// Core Solana MEV Bot Engine (Rust)
// Entry point: loads configuration, starts subsystems, and manages shutdown signals.

mod config;
mod mempool;
mod tx_builder;
mod strategy;
mod submission;
mod signer;

use log::{info, error};
use std::sync::Arc;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    env_logger::init();
    info!("Starting Solana MEV Bot core engine...");

    // Load configuration
    let cfg = match config::Config::load("../config.yaml") {
        Ok(c) => Arc::new(c),
        Err(e) => {
            error!("Failed to load config: {}", e);
            return;
        }
    };

    // Initialize subsystems
    let mut scanner = mempool::MempoolScanner::new(Arc::clone(&cfg));
    let builder_handle = tx_builder::start(Arc::clone(&cfg));
    let strategy_handle = strategy::start(Arc::clone(&cfg));
    let scanner_handle = tokio::spawn(async move { scanner.start().await });

    info!("Solana MEV Bot core engine is running.");

    // Wait for tasks (placeholder)
    let _ = tokio::join!(builder_handle, strategy_handle, scanner_handle);
}
