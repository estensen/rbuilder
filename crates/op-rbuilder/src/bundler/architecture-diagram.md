# ERC-4337 Bundler Architecture

```
┌──────────────────────────────────────────────────────────────────────────────────┐
│                              op-rbuilder                                          │
│                                                                                   │
│  ┌───────────────────┐     ┌────────────────────────┐     ┌───────────────────┐  │
│  │                   │     │                        │     │                   │  │
│  │    Transaction    │     │    OpPayloadBuilder    │     │    BlockBuilder   │  │
│  │       Pool        │     │                        │     │                   │  │
│  │                   │     │                        │     │                   │  │
│  └─────────┬─────────┘     └────────────┬───────────┘     └────────┬──────────┘  │
│            │                            │                          │             │
│            │       ┌──────────────────┐ │                          │             │
│            │       │                  │ │                          │             │
│            │       │ BundlerIntegration│ │                          │             │
│            │       │                  │ │                          │             │
│            │       └────────┬─────────┘ │                          │             │
│            │                │           │                          │             │
│            │                ▼           │                          │             │
│            │       ┌─────────────────┐  │                          │             │
│            │       │                 │  │                          │             │
│            │       │   Bundler Trait │  │                          │             │
│            │       │                 │  │                          │             │
│            │       └────────┬────────┘  │                          │             │
│            │                │           │                          │             │
│            │                ▼           │                          │             │
│            │       ┌─────────────────┐  │                          │             │
│            │       │                 │  │                          │             │
│            │       │  Bundle Menu    │  │                          │             │
│            │       │                 │  │                          │             │
│            │       └─────────────────┘  │                          │             │
│            │                ▲           │                          │             │
│            │                │           │                          │             │
│            │       ┌─────────────────┐  │                          │             │
│            │       │                 │  │                          │             │
│            │       │  ProfitOracle   │  │                          │             │
│            │       │                 │  │                          │             │
│            │       └─────────────────┘  │                          │             │
│            │                            │                          │             │
│            ▼                            ▼                          ▼             │
│  ┌───────────────────────────────────────────────────────────────────────────┐  │
│  │                                                                           │  │
│  │                       Block Assembly Pipeline                             │  │
│  │                                                                           │  │
│  └───────────────────────────────────────────────────────────────────────────┘  │
│                                                                                   │
└──────────────────────────────────────────────────────────────────────────────────┘
```

## Component Descriptions

### 1. BundlerIntegration
The integration layer between op-rbuilder and the bundler implementation. It manages bundle menus and provides a clean interface for the payload builder.

### 2. Bundler Trait
Defines the interface for bundler implementations. This allows for mock bundlers during development and real bundlers in production.

### 3. Bundle Menu
A collection of bundle options with different characteristics (gas usage, profit) that the payload builder can choose from.

### 4. Profit Calculation
Bundles include a profit hint that helps rank their expected profit contribution to the block.

### 5. Block Assembly Pipeline
Where regular transactions and bundled UserOperations are combined into a final block.

## Data Flow

1. The payload builder requests transactions from both the transaction pool and the bundler
2. The bundler integration layer requests a menu of bundle options from the bundler
3. The profit oracle ranks the bundles by expected profit
4. The payload builder selects the most profitable bundle that fits in the available gas
5. The selected bundle is included in the block alongside regular transactions

## Design Principles

1. **Separation of Concerns**
   - Bundler interface is decoupled from implementation
   - Bundle menu generation is separate from bundle selection

2. **Feature Isolation**
   - The entire bundler system is behind a feature flag
   - No impact on regular operation when disabled

3. **Extensibility**
   - Easy to plug in different bundler implementations
   - Different profit oracles for different valuation strategies