# ERC-4337 Bundler Integration for op-rbuilder

This directory implements the ERC-4337 bundler integration component for op-rbuilder, which enables account abstraction support.

## Overview

The bundler integration allows op-rbuilder to include ERC-4337 UserOperation bundles in blocks it builds alongside regular transactions. This is done by implementing a "bundle menu" approach that provides multiple bundle options with different characteristics.

## Usage

Enable the bundler integration with:

```bash
cargo run -p op-rbuilder --bin op-rbuilder --features aa4337 -- node \
    --builder.enable-4337 \
    <other-standard-options>
```

## Architecture

The bundler integration consists of several key components:

1. **Bundler Trait**: Defines the interface for bundler implementations
2. **MockBundler**: A simple mock implementation that returns hardcoded bundles
3. **BundlerIntegration**: Integrates the bundler with the payload builder
4. **ProfitOracle**: Evaluates bundle profitability

## Bundle Menu System

The "bundle menu" approach provides several advantages:

- Offers multiple bundle options with different gas usage
- Allows the builder to select the most profitable bundle that fits
- Prevents wasted effort by pre-simulating bundles
- Provides profit hints to guide selection

## Testing

Unit tests are provided for all components, and the bundler can be tested easily by running with the `aa4337` feature enabled.

## Future Work

- Implement a real bundler adapter using Rundler
- Add storage layer for bundle simulation
- Implement advanced profit calculation with token pricing
- Add metrics for bundler performance