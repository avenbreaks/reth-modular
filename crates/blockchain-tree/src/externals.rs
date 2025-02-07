//! Blockchain tree externals.

use reth_db::database::Database;
use reth_interfaces::consensus::Consensus;
use reth_primitives::ChainSpec;
use reth_provider::ProviderFactory;
use std::sync::Arc;

/// A container for external components.
///
/// This is a simple container for external components used throughout the blockchain tree
/// implementation:
///
/// - A handle to the database
/// - A handle to the consensus engine
/// - The executor factory to execute blocks with
/// - The chain spec
#[derive(Debug)]
pub struct TreeExternals<DB, EF> {
    /// The database, used to commit the canonical chain, or unwind it.
    pub(crate) db: DB,
    /// The consensus engine.
    pub(crate) consensus: Arc<dyn Consensus>,
    /// The executor factory to execute blocks with.
    pub(crate) executor_factory: EF,
    /// The chain spec.
    pub(crate) chain_spec: Arc<ChainSpec>,
}

impl<DB, EF> TreeExternals<DB, EF> {
    /// Create new tree externals.
    pub fn new(
        db: DB,
        consensus: Arc<dyn Consensus>,
        executor_factory: EF,
        chain_spec: Arc<ChainSpec>,
    ) -> Self {
        Self { db, consensus, executor_factory, chain_spec }
    }
}

impl<DB: Database, EF> TreeExternals<DB, EF> {
    /// Return shareable database helper structure.
    pub fn database(&self) -> ProviderFactory<&DB> {
        ProviderFactory::new(&self.db, self.chain_spec.clone())
    }
}
