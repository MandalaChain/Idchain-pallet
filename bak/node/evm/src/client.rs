//! Module for defining client and runtime API collections.

// Substrate
use sc_executor::{NativeElseWasmExecutor, NativeExecutionDispatch, NativeVersion};
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
// Local
use evm_runtime::{opaque::Block, AccountId, Balance, Nonce};

use crate::eth::EthCompatRuntimeApiCollection;

/// Full backend type for the node.
pub type FullBackend = sc_service::TFullBackend<Block>;

/// Full client type for the node.
pub type FullClient<RuntimeApi, Executor> =
    sc_service::TFullClient<Block, RuntimeApi, NativeElseWasmExecutor<Executor>>;

/// Client type for the node.
pub type Client = FullClient<evm_runtime::RuntimeApi, RuntimeExecutor>;

/// Host functions for benchmarking.
/// Only enable the benchmarking host functions when we actually want to benchmark.
#[cfg(feature = "runtime-benchmarks")]
pub type HostFunctions = frame_benchmarking::benchmarking::HostFunctions;

/// Otherwise, we use empty host functions for external host functions.
#[cfg(not(feature = "runtime-benchmarks"))]
pub type HostFunctions = ();

/// Executor for the runtime, used to execute native and WASM code.
pub struct RuntimeExecutor;

impl NativeExecutionDispatch for RuntimeExecutor {
    type ExtendHostFunctions = HostFunctions;

    /// Dispatches a method call to the runtime.
    fn dispatch(method: &str, data: &[u8]) -> Option<Vec<u8>> {
        evm_runtime::api::dispatch(method, data)
    }

    /// Returns the native version of the runtime.
    fn native_version() -> NativeVersion {
        evm_runtime::native_version()
    }
}

/// A set of APIs that every runtime must implement.
pub trait BaseRuntimeApiCollection:
    sp_api::ApiExt<Block>
    + sp_api::Metadata<Block>
    + sp_block_builder::BlockBuilder<Block>
    + sp_offchain::OffchainWorkerApi<Block>
    + sp_session::SessionKeys<Block>
    + sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block>
{
}

impl<Api> BaseRuntimeApiCollection for Api where
    Api: sp_api::ApiExt<Block>
        + sp_api::Metadata<Block>
        + sp_block_builder::BlockBuilder<Block>
        + sp_offchain::OffchainWorkerApi<Block>
        + sp_session::SessionKeys<Block>
        + sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block>
{
}

/// A set of APIs that the template runtime must implement.
pub trait RuntimeApiCollection:
    BaseRuntimeApiCollection
    + EthCompatRuntimeApiCollection
    + sp_consensus_aura::AuraApi<Block, AuraId>
    + sp_consensus_grandpa::GrandpaApi<Block>
    + frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Nonce>
    + pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance>
{
}

impl<Api> RuntimeApiCollection for Api where
    Api: BaseRuntimeApiCollection
        + EthCompatRuntimeApiCollection
        + sp_consensus_aura::AuraApi<Block, AuraId>
        + sp_consensus_grandpa::GrandpaApi<Block>
        + frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Nonce>
        + pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance>
{
}
