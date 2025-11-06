// Configuration loader for the MEV bot.
// Parses YAML config file and provides typed access to runtime parameters.

use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize, Clone)]
pub struct RpcNode {
    pub url: String,
    pub priority: Option<u8>,
    pub latency_ms: Option<u32>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Strategies {
    pub arbitrage: Option<StrategyToggle>,
    pub sandwich: Option<StrategyToggle>,
    pub liquidation: Option<StrategyToggle>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct StrategyToggle {
    pub enabled: Option<bool>,
    pub min_profit_usd: Option<f64>,
    pub max_slippage: Option<f64>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PriorityFee {
    pub base_compute_unit_price: Option<f64>,
    pub dynamic_adjustment: Option<bool>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub rpc_nodes: Vec<RpcNode>,
    pub wallet: serde_yaml::Value,
    pub strategies: Option<Strategies>,
    pub priority_fee: Option<PriorityFee>,
    pub monitoring: Option<serde_yaml::Value>,
}

impl Config {
    pub fn load(path: &str) -> anyhow::Result<Config> {
        let data = fs::read_to_string(path)?;
        let cfg: Config = serde_yaml::from_str(&data)?;
        Ok(cfg)
    }
}
