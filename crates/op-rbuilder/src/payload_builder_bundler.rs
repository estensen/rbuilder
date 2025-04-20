use crate::bundler::{BundleMeta, Bundler, MockBundler};
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// BundlerIntegration provides integration with ERC-4337 bundler
pub struct BundlerIntegration {
    /// The bundler instance
    bundler: Arc<dyn Bundler>,
    /// Currently selected bundle menu
    current_menu: RwLock<Vec<BundleMeta>>,
}

impl Default for BundlerIntegration {
    fn default() -> Self {
        Self::new()
    }
}

impl BundlerIntegration {
    /// Create a new bundler integration
    pub fn new() -> Self {
        info!("Initializing ERC-4337 bundler integration");
        Self {
            bundler: Arc::new(MockBundler::new()),
            current_menu: RwLock::new(Vec::new()),
        }
    }

    /// Get the current bundle menu
    pub async fn get_bundle_menu(&self) -> Vec<BundleMeta> {
        self.current_menu.read().await.clone()
    }

    /// Request a new bundle menu from the bundler
    pub async fn request_bundle_menu(&self, gas_limit: u64, fee_target: Option<i128>) -> Vec<BundleMeta> {
        // Request k=3 bundle options from the bundler
        let menu = self.bundler
            .propose_bundles(gas_limit, 3, fee_target)
            .await;
        
        if menu.is_empty() {
            debug!("Bundler returned empty menu");
        } else {
            let mut lock = self.current_menu.write().await;
            *lock = menu.clone();
            
            info!(
                "Received bundle menu with {} options, gas_usages: {:?}, profits: {:?}",
                menu.len(),
                menu.iter().map(|b| b.gas_used).collect::<Vec<_>>(),
                menu.iter().map(|b| b.profit_hint).collect::<Vec<_>>()
            );
        }
        
        menu
    }

    /// Find the best bundle option for the given gas limit
    pub async fn find_best_bundle(&self, gas_limit: u64) -> Option<BundleMeta> {
        let menu = self.current_menu.read().await;
        if menu.is_empty() {
            return None;
        }

        // Find the bundle with the highest profit that fits in the gas limit
        menu.iter()
            .filter(|b| b.gas_used <= gas_limit)
            .max_by_key(|b| b.profit_hint)
            .cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_bundler_integration() {
        let integration = BundlerIntegration::new();
        
        // Test with sufficient gas
        let menu = integration.request_bundle_menu(5_000_000, None).await;
        assert_eq!(menu.len(), 3);
        
        // Get the best bundle
        let best = integration.find_best_bundle(5_000_000).await;
        assert!(best.is_some());
        let best = best.unwrap();
        assert_eq!(best.gas_used, 3_000_000);
        assert_eq!(best.profit_hint, 800_000_000_000_000);
        
        // Test with limited gas
        let menu = integration.request_bundle_menu(2_000_000, None).await;
        assert_eq!(menu.len(), 2);
        
        // Get the best bundle with limited gas
        let best = integration.find_best_bundle(2_000_000).await;
        assert!(best.is_some());
        let best = best.unwrap();
        assert_eq!(best.gas_used, 1_000_000);
        assert_eq!(best.profit_hint, 300_000_000_000_000);
    }
}