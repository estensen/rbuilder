use op_alloy_consensus::OpTxEnvelope;

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
