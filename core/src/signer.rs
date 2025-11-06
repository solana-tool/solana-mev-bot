// Signing utilities.
// Load keypair from disk or HSM and sign transactions with low-latency signing pipeline.

use log::info;

pub struct Signer {}

impl Signer {
    pub fn new() -> Self {
        Signer {}
    }

    pub fn sign_transaction(&self, _tx: &[u8]) -> Vec<u8> {
        info!("Signing transaction (placeholder)...");
        // TODO: integrate with Solana keypair signing: ed25519
        vec![]
    }
}
