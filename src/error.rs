//! Error types for the integration demo

use thiserror::Error;

#[derive(Error, Debug)]
pub enum IntegrationError {
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("TipRouter consensus error: {0}")]
    ConsensusError(String),
    
    #[error("Restaking validation error: {0}")]
    RestakingError(String),
    
    #[error("Reward processing error: {0}")]
    RewardError(String),
    
    #[error("Network error: {0}")]
    NetworkError(String),
}

pub type Result<T> = std::result::Result<T, IntegrationError>;