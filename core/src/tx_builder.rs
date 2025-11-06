// Transaction builder and submission orchestrator.
// Responsible for constructing atomic multi-instruction transactions,
// estimating compute units, setting priority fees, and submitting to validator leaders.

use std::sync::Arc;
use crate::config::Config;
use log::info;
use tokio::task::JoinHandle;

pub fn start(config: Arc<Config>) -> JoinHandle<()> {
    tokio::spawn(async move {
        let mut builder = TxBuilder::new(config);
        builder.run_loop().await;
    })
}

pub struct TxBuilder {
    config: Arc<Config>,
}

impl TxBuilder {
    pub fn new(config: Arc<Config>) -> Self {
        TxBuilder { config }
    }

    /// Main loop for the tx builder. It should receive strategy proposals, construct transactions,
    /// call simulation endpoints, and then submit if profitable.
    pub async fn run_loop(&mut self) {
        info!("Transaction builder started (placeholder).");

        // TODO:
        // - Receive proposed operations from strategy engine via in-memory queue (mpsc)
        // - Build Transaction with multiple Instructions
        // - use `simulateTransaction` RPC call to verify success & profit
        // - determine compute_unit_price and set priority fee
        // - sign transaction using local keypair (see signer module)
        // - submit to validator leader (see submission module)
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
        }
    }
}
