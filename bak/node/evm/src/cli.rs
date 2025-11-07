use crate::service::EthConfiguration;

/// Available sealing methods for the node.
#[derive(Copy, Clone, Debug, Default, clap::ValueEnum)]
pub enum Sealing {
    /// Seal using RPC method.
    #[default]
    Manual,
    /// Seal instantly when a transaction is executed.
    Instant,
}

/// Command-line interface (CLI) for the node.
#[derive(Debug, clap::Parser)]
pub struct Cli {
    /// Optional subcommand to execute.
    #[command(subcommand)]
    pub subcommand: Option<Subcommand>,

    #[allow(missing_docs)]
    #[command(flatten)]
    pub run: sc_cli::RunCmd,

    /// Choose sealing method.
    #[arg(long, value_enum, ignore_case = true)]
    pub sealing: Option<Sealing>,

    /// Ethereum compatibility configuration.
    #[command(flatten)]
    pub eth: EthConfiguration,
}

/// Enum representing the available subcommands.
#[derive(Debug, clap::Subcommand)]
pub enum Subcommand {
    /// Key management CLI utilities.
    #[command(subcommand)]
    Key(sc_cli::KeySubcommand),

    /// Build a chain specification.
    BuildSpec(sc_cli::BuildSpecCmd),

    /// Validate blocks.
    CheckBlock(sc_cli::CheckBlockCmd),

    /// Export blocks.
    ExportBlocks(sc_cli::ExportBlocksCmd),

    /// Export the state of a given block into a chain spec.
    ExportState(sc_cli::ExportStateCmd),

    /// Import blocks.
    ImportBlocks(sc_cli::ImportBlocksCmd),

    /// Remove the whole chain.
    PurgeChain(sc_cli::PurgeChainCmd),

    /// Revert the chain to a previous state.
    Revert(sc_cli::RevertCmd),

    /// Sub-commands concerned with benchmarking.
    #[cfg(feature = "runtime-benchmarks")]
    #[command(subcommand)]
    Benchmark(frame_benchmarking_cli::BenchmarkCmd),

    /// Sub-commands concerned with benchmarking (not available without benchmarking feature).
    #[cfg(not(feature = "runtime-benchmarks"))]
    Benchmark,

    /// Database meta columns information.
    FrontierDb(fc_cli::FrontierDbCmd),

    /// Display chain information.
    ChainInfo(sc_cli::ChainInfoCmd),
}
