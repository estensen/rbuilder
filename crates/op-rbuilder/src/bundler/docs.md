# ERC-4337 Bundler Integration for op-rbuilder

## Architecture Overview

```
┌────────────────────────────────────────────────────────────────────────┐
│                            op-rbuilder                                  │
│                                                                         │
│  ┌────────────────┐     ┌────────────────┐     ┌────────────────────┐  │
│  │                │     │                │     │                    │  │
│  │ Transaction    │     │    ERC-4337    │     │                    │  │
│  │ Pool           │◄────┤    Bundler     │     │  Payload Builder   │  │
│  │                │     │                │     │                    │  │
│  └────────────────┘     └────────┬───────┘     └─────────▲──────────┘  │
│                                  │                       │             │
│                                  │                       │             │
│                                  │                       │             │
│                          ┌───────▼───────┐      ┌────────┴───────┐     │
│                          │               │      │                │     │
│                          │ Bundle Menu  │─────►│  Block Builder  │     │
│                          │               │      │                │     │
│                          └───────────────┘      └────────────────┘     │
│                                                                         │
└────────────────────────────────────────────────────────────────────────┘
```

## Component Descriptions

### 1. ERC-4337 Bundler
The bundler component is responsible for bundling UserOperations into transactions that can be included in blocks. It implements the ERC-4337 standard for account abstraction, providing a "menu" of possible bundle options.

### 2. Bundle Menu
The bundler generates a "menu" of bundle options with different characteristics:
- Different gas usage (3M gas, 5M gas, "fill remaining")
- Different risk/return profiles (high-fee vs sponsored via Paymaster)
- Pre-simulated to ensure execution success

### 3. Block Builder
The block builder selects the optimal bundle(s) from the menu based on:
- Available gas in the block
- Expected profit (using the ProfitOracle to price transactions)
- Block space optimization

### 4. Profit Calculation
Each bundle includes a profit hint that indicates its expected contribution to block profit.

## Workflow

1. The payload builder requests candidate transactions for a new block
2. It queries both the transaction pool for regular transactions and the bundler for ERC-4337 bundles
3. The bundler returns a menu of bundle options with different gas usage and profit profiles
4. The block builder evaluates and selects the optimal combination of regular transactions and bundles
5. The final block is constructed and proposed through the execution layer

## Benefits

- Higher block profit through cross-mempool ordering
- Perfect block space utilization with different bundle sizes
- Simplified integration with existing transaction flow
- Optional feature that can be toggled without affecting regular operation

## Future Enhancements

- Integration with a full-featured bundler like Rundler
- Advanced profit calculation with token pricing
- Performance optimizations for bundle simulation
- Monitoring and metrics for bundle performance