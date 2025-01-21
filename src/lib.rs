//! wIndexer Jito Integration Demo
//! Demonstrates integration between wIndexer and Jito's TipRouter NCN

pub mod error;
pub mod integration;
pub mod types;
pub mod monitoring;

pub use integration::WindexerJitoIntegration;
pub use types::{IntegrationConfig, ValidationResult};

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use types::IndexData;

    #[tokio::test]
    async fn test_basic_validation_flow() {
        let config = IntegrationConfig::new_local();
        let integration = WindexerJitoIntegration::new(config).await.unwrap();

        let test_data = IndexData {
            slot: 100,
            block_hash: [0u8; 32],
            parent_slot: 99,
            timestamp: Utc::now().timestamp(),
            transaction_count: 1000,
        };

        let result = integration.validate_data(test_data).await;
        assert!(result.is_ok(), "Validation should succeed");
        
        let validation = result.unwrap();
        assert!(validation.is_valid, "Data should be valid");
        assert!(validation.consensus_percentage >= 67.0, "Should meet consensus threshold");
    }
}