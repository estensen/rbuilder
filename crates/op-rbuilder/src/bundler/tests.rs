//! Tests for the bundler module

#[cfg(test)]
mod tests {
    use crate::bundler::{Bundler, MockBundler};
    use op_rbuilder::payload_builder_bundler::BundlerIntegration;

    #[tokio::test]
    async fn test_mock_bundler_basic() {
        let bundler = MockBundler::new();
        
        // Test with sufficient gas
        let bundles = bundler
            .propose_bundles(5_000_000, 3, None)
            .await;
        
        assert_eq!(bundles.len(), 3);
        assert!(bundles[0].gas_used <= 5_000_000);
        assert!(bundles[1].gas_used <= 5_000_000);
        assert!(bundles[2].gas_used <= 5_000_000);
        
        // Test with limited gas
        let bundles = bundler
            .propose_bundles(800_000, 3, None)
            .await;
        
        assert_eq!(bundles.len(), 1);
        assert!(bundles[0].gas_used <= 800_000);
        
        // Test with insufficient gas
        let bundles = bundler
            .propose_bundles(100_000, 3, None)
            .await;
        
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
    
    #[tokio::test]
    async fn test_bundler_integration() {
        let integration = BundlerIntegration::new();
        
        // Test requesting a bundle menu
        let menu = integration.request_bundle_menu(5_000_000, None).await;
        assert_eq!(menu.len(), 3);
        
        // Test menu is stored
        let stored_menu = integration.get_bundle_menu().await;
        assert_eq!(stored_menu.len(), 3);
        
        // Test finding best bundle
        let best = integration.find_best_bundle(5_000_000).await;
        assert!(best.is_some());
        let best = best.unwrap();
        assert_eq!(best.gas_used, 3_000_000);
        assert_eq!(best.profit_hint, 800_000_000_000_000);
        
        // Test with gas limit that excludes largest bundle
        let best = integration.find_best_bundle(2_000_000).await;
        assert!(best.is_some());
        let best = best.unwrap();
        assert_eq!(best.gas_used, 1_000_000);
        assert_eq!(best.profit_hint, 300_000_000_000_000);
        
        // Test with insufficient gas
        let best = integration.find_best_bundle(100_000).await;
        assert!(best.is_none());
    }
}