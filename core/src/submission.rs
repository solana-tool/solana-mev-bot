// Submission layer.
// Responsible for leader-aware submission of signed transactions, including private node routing,
// and handling responses (confirmation, errors, reorgs).

use std::sync::Arc;
use crate::config::Config;
use log::info;

pub struct Submitter {
    config: Arc<Config>,
}

impl Submitter {
    pub fn new(config: Arc<Config>) -> Self {
        Submitter { config }
    }

    pub async fn submit_transaction(&self, signed_tx: Vec<u8>) -> anyhow::Result<String> {
        info!("Submitting transaction (placeholder)...");
        // TODO: Implement direct RPC post to leader/validator or use a private relay.
        Ok(String::from("signature_placeholder"))
    }
}
