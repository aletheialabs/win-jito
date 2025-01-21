
use {
    anyhow::Result,
    chrono::Utc,
    log::{info, error},
    windexer_jito_demo::{
        IntegrationConfig,
        WindexerJitoIntegration,
        types::IndexData,
    },
};

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    info!("Starting wIndexer-Jito integration demo");

    let config = IntegrationConfig::new_local();
    
    let integration = WindexerJitoIntegration::new(config).await?;

    let test_data = IndexData {
        slot: 100,
        block_hash: [0u8; 32],
        parent_slot: 99,
        timestamp: Utc::now().timestamp(),
        transaction_count: 1000,
    };

    info!("Submitting test data for validation");
    match integration.validate_data(test_data).await {
        Ok(result) => {
            info!("Validation successful!");
            info!("Consensus achieved: {:.2}%", result.consensus_percentage);
            info!("Participating stake: {} SOL", result.participating_stake);
        }
        Err(e) => {
            error!("Validation failed: {}", e);
        }
    }

    Ok(())
}