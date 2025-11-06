// Strategy engine.
// Contains core logic for arbitrage, sandwich, and liquidation detection.
// The engine listens to mempool events and proposes transactions to the TxBuilder.

use std::sync::Arc;
use crate::config::Config;
use log::info;
use tokio::task::JoinHandle;

pub fn start(config: Arc<Config>) -> JoinHandle<()> {
    tokio::spawn(async move {
        let mut engine = StrategyEngine::new(config);
        engine.run().await;
    })
}

pub struct StrategyEngine {
    config: Arc<Config>,
}

impl StrategyEngine {
    pub fn new(config: Arc<Config>) -> Self {
        StrategyEngine { config }
    }

    /// Main strategy loop.
    /// Parse events, detect opportunities, compute expected profit, and forward proposals.
    pub async fn run(&mut self) {
        info!("Strategy engine started (placeholder).");

        // TODO:
        // - Implement DEX price aggregation (Raydium, Orca, Saber, Jupiter)
        // - Compute arbitrage graphs and triangular arbitrage
        // - Detect sandwich opportunities from large pending swaps
        // - Detect liquidation opportunities on target lending platforms
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
        }
    }
}
