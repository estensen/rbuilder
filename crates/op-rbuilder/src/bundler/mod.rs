//! ERC-4337 bundler integration
//!
//! This module implements a lightweight ERC-4337 bundler interface that provides
//! a menu of transaction bundles to the payload builder.

pub use types::BundleMeta;

pub mod integration;
pub use integration::BundlerIntegration;

pub mod mock;
pub mod types;
pub use mock::MockBundler;
