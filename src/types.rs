//! Type definitions for the integration demo

use {
    serde::{Deserialize, Serialize},
    solana_sdk::pubkey::Pubkey,
    std::collections::HashMap,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationConfig {
    pub tiprouter_config: TiprouterConfig,
    pub restaking_config: RestakingConfig,
    pub reward_config: RewardConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TiprouterConfig {
    pub program_id: Pubkey,
    pub stake_threshold: u64,
    pub consensus_threshold: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestakingConfig {
    pub vault_program_id: Pubkey,
    pub min_stake: u64,
    pub max_stake: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardConfig {
    pub base_rate: f64,
    pub performance_multiplier: f64,
    pub distribution_frequency: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub consensus_percentage: f64,
    pub participating_stake: u64,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexData {
    pub slot: u64,
    pub block_hash: [u8; 32],
    pub parent_slot: u64,
    pub timestamp: i64,
    pub transaction_count: u64,
}

impl IntegrationConfig {
    pub fn new_local() -> Self {
        Self {
            tiprouter_config: TiprouterConfig {
                program_id: Pubkey::new_unique(),
                stake_threshold: 1_000_000,
                consensus_threshold: 67,
            },
            restaking_config: RestakingConfig {
                vault_program_id: Pubkey::new_unique(),
                min_stake: 100_000,
                max_stake: 10_000_000,
            },
            reward_config: RewardConfig {
                base_rate: 0.05,
                performance_multiplier: 1.0,
                distribution_frequency: 100,
            },
        }
    }
}