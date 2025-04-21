//! ERC-4337 bundler integration for op-rbuilder
//!
//! This module implements a lightweight ERC-4337 bundler interface that provides
//! a menu of transaction bundles to the payload builder. The bundler is enabled
//! via the `aa4337` feature flag.

use alloy_primitives::Signature;
use futures::future::BoxFuture;
use op_alloy_consensus::{OpTxEnvelope, OpTypedTransaction};
use tracing::info;

/// Metadata for a bundle transaction
#[derive(Debug, Clone)]
pub struct BundleMeta {
    /// The wrapped handleOps() transaction
    pub tx: OpTxEnvelope,
    /// Simulated gas used by the bundle
    pub gas_used: u64,
    /// Profit hint in wei (can be negative)
    pub profit_hint: i128,
}

/// Bundler trait for proposing bundles of transactions
pub trait Bundler: Send + Sync {
    /// Propose a menu of bundles based on the given constraints
    ///
    /// # Arguments
    /// * `gas_limit` - Residual gas available for bundles
    /// * `k` - Maximum number of bundle options to return
    /// * `fee_target` - Optional minimum fee target (only return bundles with profit >= fee_target)
    fn propose_bundles(
        &self,
        gas_limit: u64,
        k: usize,
        fee_target: Option<i128>,
    ) -> BoxFuture<'_, Vec<BundleMeta>>;
}

/// Mock bundler implementation with hardcoded responses
pub struct MockBundler;

impl MockBundler {
    /// Create a new mock bundler
    pub fn new() -> Self {
        info!("Initializing ERC-4337 mock bundler");
        Self
    }
}

impl Bundler for MockBundler {
    fn propose_bundles(
        &self,
        gas_limit: u64,
        k: usize,
        fee_target: Option<i128>,
    ) -> BoxFuture<'_, Vec<BundleMeta>> {
        Box::pin(async move {
            info!(
                "Proposing bundles with gas_limit={}, k={}, fee_target={:?}",
                gas_limit, k, fee_target
            );

            // Create a few mock bundles with different gas usage and profit
            let mut bundles = Vec::new();

            if gas_limit < 500_000 {
                // Not enough gas for any bundle
                return bundles;
            }

            // Mock bundle sizes
            let sizes = [
                (500_000, 150_000_000_000_000_i128), // 0.5M gas, 0.00015 ETH profit
                (1_000_000, 300_000_000_000_000_i128), // 1M gas, 0.0003 ETH profit
                (3_000_000, 800_000_000_000_000_i128), // 3M gas, 0.0008 ETH profit
            ];

            for (_i, (gas, profit)) in sizes.iter().enumerate() {
                if *gas <= gas_limit && bundles.len() < k {
                    // Only include bundles that meet the fee target
                    if let Some(target) = fee_target {
                        if *profit < target {
                            continue;
                        }
                    }

                    // Create a mock transaction (would be a real handleOps() in production)
                    let tx = OpTypedTransaction::Eip1559(Default::default());

                    let enveloped: OpTxEnvelope = (tx, Signature::test_signature()).into();

                    bundles.push(BundleMeta {
                        tx: enveloped,
                        gas_used: *gas,
                        profit_hint: *profit,
                    });
                }
            }

            info!("Proposed {} bundle options", bundles.len());
            bundles
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_bundler_propose_bundles() {
        let bundler = MockBundler::new();

        // Test with sufficient gas
        let bundles = bundler.propose_bundles(5_000_000, 3, None).await;
        assert_eq!(bundles.len(), 3);

        // Test with limited gas
        let bundles = bundler.propose_bundles(800_000, 3, None).await;
        assert_eq!(bundles.len(), 1);

        // Test with fee target
        let bundles = bundler
            .propose_bundles(5_000_000, 3, Some(500_000_000_000_000))
            .await;
        assert_eq!(bundles.len(), 1);

        // Test with insufficient gas
        let bundles = bundler.propose_bundles(400_000, 3, None).await;
        assert_eq!(bundles.len(), 0);
    }
}
