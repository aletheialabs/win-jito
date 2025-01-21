//! Main integration implementation

use {
    crate::{
        error::Result,
        types::*,
        monitoring::MetricsCollector,
    },
    log::info,
    solana_sdk::pubkey::Pubkey,
    std::sync::Arc,
    tokio::sync::RwLock,
};

pub struct WindexerJitoIntegration {
    tiprouter: Arc<TiprouterManager>,
    restaking: Arc<RestakingManager>,
    rewards: Arc<RewardManager>,
    metrics: Arc<MetricsCollector>,
}

pub struct TiprouterManager {
    config: TiprouterConfig,
    state: RwLock<TiprouterState>,
}

pub struct RestakingManager {
    config: RestakingConfig,
    state: RwLock<RestakingState>,
}

pub struct RewardManager {
    config: RewardConfig,
    state: RwLock<RewardState>,
}

#[derive(Debug)]
struct TiprouterState {
    consensus_count: u64,
    last_consensus: Option<ValidationResult>,
}

#[derive(Debug)]
struct RestakingState {
    current_stake: u64,
}

#[derive(Debug)] 
struct RewardState {
    total_rewards: u64,
    last_distribution: u64,
}

impl WindexerJitoIntegration {
    pub async fn new(config: IntegrationConfig) -> Result<Self> {
        let tiprouter = Arc::new(TiprouterManager::new(config.tiprouter_config));
        let restaking = Arc::new(RestakingManager::new(config.restaking_config));
        let rewards = Arc::new(RewardManager::new(config.reward_config));
        let metrics = Arc::new(MetricsCollector::new());

        Ok(Self {
            tiprouter,
            restaking, 
            rewards,
            metrics,
        })
    }

    pub async fn validate_data(&self, data: IndexData) -> Result<ValidationResult> {
        info!("Starting validation for slot {}", data.slot);
        
        self.metrics.record_validation_attempt(&data).await;

        let consensus = self.tiprouter.get_consensus(&data).await?;
        
        self.restaking.verify_stake(&consensus).await?;
        
        if consensus.is_valid {
            self.rewards.process_rewards(&consensus).await?;
            self.metrics.record_successful_validation(&consensus).await;
        }

        Ok(consensus)
    }
}

impl TiprouterManager {
    pub fn new(config: TiprouterConfig) -> Self {
        Self {
            config,
            state: RwLock::new(TiprouterState {
                consensus_count: 0,
                last_consensus: None,
            }),
        }
    }

    pub async fn get_consensus(&self, _data: &IndexData) -> Result<ValidationResult> {
        let mut state = self.state.write().await;
        
        let consensus_percentage = 75.0;
        let participating_stake = self.config.stake_threshold;

        let result = ValidationResult {
            is_valid: consensus_percentage >= f64::from(self.config.consensus_threshold),
            consensus_percentage,
            participating_stake,
            metadata: Default::default(),
        };

        state.last_consensus = Some(result.clone());
        state.consensus_count += 1;

        Ok(result)
    }
}

impl RestakingManager {
    pub fn new(config: RestakingConfig) -> Self {
        Self {
            config,
            state: RwLock::new(RestakingState {
                current_stake: 0,
            }),
        }
    }

    pub async fn verify_stake(&self, consensus: &ValidationResult) -> Result<()> {
        let _state = self.state.read().await;
        
        if consensus.participating_stake >= self.config.min_stake {
            info!("Stake verification passed: {} >= {}", 
                consensus.participating_stake, 
                self.config.min_stake
            );
            Ok(())
        } else {
            Err(crate::error::IntegrationError::RestakingError(
                "Insufficient stake".to_string()
            ))
        }
    }
}

impl RewardManager {
    pub fn new(config: RewardConfig) -> Self {
        Self {
            config,
            state: RwLock::new(RewardState {
                total_rewards: 0,
                last_distribution: 0,
            }),
        }
    }

    pub async fn process_rewards(&self, consensus: &ValidationResult) -> Result<()> {
        let mut state = self.state.write().await;
        
        let base_reward = (consensus.participating_stake as f64 * self.config.base_rate) as u64;
        let performance_bonus = (base_reward as f64 * self.config.performance_multiplier) as u64;
        let total = base_reward + performance_bonus;
        
        state.total_rewards += total;
        state.last_distribution = consensus.participating_stake;

        info!("Processed rewards: {} base + {} bonus = {} total", 
            base_reward,
            performance_bonus,
            total
        );
        
        Ok(())
    }
}