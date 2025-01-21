//! Monitoring and metrics collection

use {
    crate::types::*,
    chrono::{DateTime, Utc},
    dashmap::DashMap,
    tokio::sync::RwLock,
};

pub struct MetricsCollector {
    validation_attempts: RwLock<u64>,
    successful_validations: RwLock<u64>,
    validation_times: DashMap<u64, DateTime<Utc>>,
}

impl MetricsCollector {
    pub fn new() -> Self {
        Self {
            validation_attempts: RwLock::new(0),
            successful_validations: RwLock::new(0),
            validation_times: DashMap::new(),
        }
    }

    pub async fn record_validation_attempt(&self, data: &IndexData) {
        let mut attempts = self.validation_attempts.write().await;
        *attempts += 1;
        self.validation_times.insert(data.slot, Utc::now());
    }

    pub async fn record_successful_validation(&self, _result: &ValidationResult) {
        let mut successes = self.successful_validations.write().await;
        *successes += 1;
    }
}