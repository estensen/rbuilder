//! ERC-4337 bundler integration
//!
//! This module implements a lightweight ERC-4337 bundler interface that provides
//! a menu of transaction bundles to the payload builder. The bundler is enabled
//! via the `aa4337` feature flag.

pub use types::BundleMeta;

pub mod integration;
pub use integration::BundlerIntegration;

pub mod mock;
pub mod types;
pub use mock::MockBundler;

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
    ) -> Vec<BundleMeta>;
}
