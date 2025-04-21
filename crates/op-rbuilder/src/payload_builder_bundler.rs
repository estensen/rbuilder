use crate::bundler::{BundleMeta, Bundler, MockBundler};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info};

// Use a constant for menu size
const DEFAULT_MENU_SIZE: usize = 3;

/// BundlerIntegration provides integration with ERC-4337 bundler
pub struct BundlerIntegration {
    /// The bundler instance
    bundler: Arc<dyn Bundler>,
    /// Currently selected bundle menu
    current_menu: RwLock<Vec<BundleMeta>>,
}

impl std::fmt::Debug for BundlerIntegration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BundlerIntegration")
            .field("bundler", &"dyn Bundler")
            .field("current_menu", &"RwLock<Vec<BundleMeta>>")
            .finish()
    }
}

impl Clone for BundlerIntegration {
    fn clone(&self) -> Self {
        Self {
            bundler: self.bundler.clone(),
            current_menu: RwLock::new(Vec::new()),
        }
    }
}

impl Default for BundlerIntegration {
    fn default() -> Self {
        debug!("Initializing ERC-4337 bundler integration with mock bundler");
        Self {
            bundler: Arc::new(MockBundler::new()),
            current_menu: RwLock::new(Vec::new()),
        }
    }
}

impl BundlerIntegration {
    pub fn with_bundler(bundler: Arc<dyn Bundler>) -> Self {
        debug!("Initializing ERC-4337 bundler integration with provided bundler");
        Self {
            bundler,
            current_menu: RwLock::new(Vec::new()),
        }
    }

    /// Get the current bundle menu
    pub async fn get_bundle_menu(&self) -> Vec<BundleMeta> {
        self.current_menu.read().await.clone()
    }

    /// Request a new bundle menu from the bundler
    pub async fn request_bundle_menu(
        &self,
        gas_limit: u64,
        fee_target: Option<i128>,
    ) -> Vec<BundleMeta> {
        // Request bundle options from the bundler using the constant
        let menu = self
            .bundler
            .propose_bundles(gas_limit, DEFAULT_MENU_SIZE, fee_target)
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
        // Use default() instead of new()
        let integration = BundlerIntegration::default();

        // Test with sufficient gas
        let menu = integration
            .request_bundle_menu(5_000_000, None::<i128>)
            .await;
        assert_eq!(menu.len(), 3);

        // Get the best bundle
        let best = integration.find_best_bundle(5_000_000).await;
        assert!(best.is_some());
        let best = best.unwrap();
        assert_eq!(best.gas_used, 3_000_000);
        assert_eq!(best.profit_hint, 800_000_000_000_000);

        // Test with limited gas
        let menu = integration
            .request_bundle_menu(2_000_000, None::<i128>)
            .await;
        assert_eq!(menu.len(), 2);

        // Get the best bundle with limited gas
        let best = integration.find_best_bundle(2_000_000).await;
        assert!(best.is_some());
        let best = best.unwrap();
        assert_eq!(best.gas_used, 1_000_000);
        assert_eq!(best.profit_hint, 300_000_000_000_000);
    }

    #[tokio::test]
    async fn test_mock_bundler_basic() {
        let bundler = MockBundler::new();

        // Test with sufficient gas
        let bundles = bundler.propose_bundles(5_000_000, 3, None).await;

        assert_eq!(bundles.len(), 3);
        assert!(bundles[0].gas_used <= 5_000_000);
        assert!(bundles[1].gas_used <= 5_000_000);
        assert!(bundles[2].gas_used <= 5_000_000);

        // Test with limited gas
        let bundles = bundler.propose_bundles(800_000, 3, None).await;

        assert_eq!(bundles.len(), 1);
        assert!(bundles[0].gas_used <= 800_000);

        // Test with insufficient gas
        let bundles = bundler.propose_bundles(100_000, 3, None).await;

        assert_eq!(bundles.len(), 0);
    }

    #[tokio::test]
    async fn test_mock_bundler_fee_target() {
        let bundler = MockBundler::new();

        // Test with fee target that allows all bundles
        let bundles = bundler
            .propose_bundles(5_000_000, 3, Some(100_000_000_000_000))
            .await;

        assert_eq!(bundles.len(), 3);

        // Test with fee target that filters some bundles
        let bundles = bundler
            .propose_bundles(5_000_000, 3, Some(500_000_000_000_000))
            .await;

        assert_eq!(bundles.len(), 1);
        assert!(bundles[0].profit_hint >= 500_000_000_000_000);
    }
}
