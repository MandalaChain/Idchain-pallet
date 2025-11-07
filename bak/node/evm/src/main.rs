//! ID CHAIN node CLI.
#![warn(missing_docs)]

/// Module for chain spec.
pub mod chain_spec;

/// Modul for RPC.
pub mod rpc;

/// Modul for service node.
pub mod service;

/// Modul for EVM compability.
pub mod eth;

/// Modul for client node.
pub mod client;

/// Modul for CLI node.
pub mod cli;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
/// Modul for node command.
pub mod command;

fn main() -> sc_cli::Result<()> {
    command::run()
}
