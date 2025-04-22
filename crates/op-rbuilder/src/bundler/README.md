# ERC‑4337 Bundler Integration for op‑rbuilder

## Overview
This feature adds a pluggable ERC‑4337 bundler interface to op‑rbuilder, allowing UserOperations to be packaged into bundles and included alongside regular transactions in block payloads. The design emphasizes **separation of concerns**, **feature isolation**, and **extensibility**.

```
        ┌──────────────────────┐
        │  OpPayloadBuilder    │
        └─────────┬────────────┘
                  │ calls
                  ▼
┌────────────────────────────────────┐
│ BundlerIntegration                 │
│ .get_best_bundle(gas_limit, fee)   │
└────────────────────────────────────┘
```

## Integration
The builder communicates with the Bundler by pulling. With that we can isolate the builder from the Bundler.

The Bundler returns a menu given the Builder's constraints (currently gas limit and fee but this could be extended)
