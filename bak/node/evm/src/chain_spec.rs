//! Module for chain specification (chain spec).

use std::{collections::BTreeMap, marker::PhantomData};

use evm_runtime::{
    AccountId, AuraConfig, BalancesConfig, EVMChainIdConfig, EVMConfig, GrandpaConfig,
    RuntimeGenesisConfig, SudoConfig, SystemConfig, BGLT, WASM_BINARY,
};
use fp_evm::GenesisAccount;
use sc_network::config::MultiaddrWithPeerId;
use sc_service::{ChainType, Properties};
use sc_telemetry::TelemetryEndpoints;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_consensus_grandpa::AuthorityId as GrandpaId;
use sp_core::{ecdsa, Pair, Public, H160, U256};
// use sp_runtime::traits::Verify;

/// The URL for the telemetry server.
const DEFAULT_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Trait for custom chain specification properties.
pub trait CustomChainSpecProperties {
    /// Returns the WASM binary for the chain.
    fn wasm_binary() -> &'static [u8] {
        WASM_BINARY.expect("Development wasm not available")
    }

    /// Returns the token symbol.
    fn token_symbol() -> &'static str;

    /// Returns the number of token decimals.
    fn token_decimals() -> u8;

    /// Returns the EVM chain ID.
    fn evm_chain_id() -> u64;

    /// Returns the initial balance.
    fn initial_balance() -> u128;

    /// Returns the chain name.
    fn chain_name() -> &'static str;

    /// Returns the chain identifier, e.g., "dev", "local", "testnet", "mainnet".
    fn chain_identifier() -> &'static str;

    /// Returns the chain type.
    fn chain_type() -> ChainType;

    /// Extension for custom properties. Override this if you have custom chain spec properties.
    fn chain_spec_properties_ext(chainspec_prop: Properties) -> Properties {
        chainspec_prop
    }

    /// Returns the default chain specification properties.
    fn default_chain_spec_properties() -> Properties {
        let mut properties = serde_json::map::Map::new();
        properties.insert("tokenSymbol".into(), Self::token_symbol().into());
        properties.insert("tokenDecimals".into(), Self::token_decimals().into());
        properties.insert("isEthereum".into(), true.into());

        Self::chain_spec_properties_ext(properties)
    }

    /// Returns the chain specification properties.
    fn chain_spec_prop() -> Properties {
        let default = Self::default_chain_spec_properties();
        Self::chain_spec_properties_ext(default)
    }

    /// Returns the default prefunded accounts. Override this if you have custom prefunded accounts.
    fn endowed_accounts() -> Vec<AccountId> {
        vec![
            // Pandi 1
            AccountId::from(hex_literal::hex!(
                "4ef0028bb468B75B146228613d29CEFBb5181AF1"
            )),
            // Pandi 2
            AccountId::from(hex_literal::hex!(
                "dE242D3DA911275663C05F4A925d506edb40d8Db"
            )),
            // Pandi 3
            AccountId::from(hex_literal::hex!(
                "bd8b90Bf8B40E45FA9dfe27e0d8E22b3E2B2996F"
            )),
            // Pandi 4
            AccountId::from(hex_literal::hex!(
                "CF45e0788bB1b2F93ea188415f4C1f16e0a97c27"
            )),
            // Pandi 5
            AccountId::from(hex_literal::hex!(
                "ED509Bb407a82e23CB8135401406C18841725Da3"
            )),
            // Below are development accounts
            // Balthar
            AccountId::from(hex_literal::hex!(
                "3Cd0A705a2DC65e5b1E1205896BaA2be8A07c6e0"
            )),
            // Charleth
            AccountId::from(hex_literal::hex!(
                "798d4Ba9baf0064Ec19eB4F0a1a45785ae9D6DFc"
            )),
            // Dorothy
            AccountId::from(hex_literal::hex!(
                "773539d4Ac0e786233D90A233654ccEE26a613D9"
            )),
            // Ethan
            AccountId::from(hex_literal::hex!(
                "Ff64d3F6efE2317EE2807d223a0Bdc4c0c49dfDB"
            )),
            // Faith
            AccountId::from(hex_literal::hex!(
                "C0F0f4ab324C46e55D02D0033343B4Be8A55532d"
            )),
        ]
    }

    /// Returns the initial authorities.
    fn initial_authorities() -> Vec<(AuraId, GrandpaId)> {
        vec![authority_keys_from_seed("Alice")]
    }

    /// Returns whether to enable println.
    fn enable_println() -> bool {
        true
    }

    /// Returns the root key.
    fn root_key() -> AccountId {
        // Balthar
        AccountId::from(hex_literal::hex!(
            "3Cd0A705a2DC65e5b1E1205896BaA2be8A07c6e0"
        ))
    }

    /// Returns the runtime genesis configuration.
    fn runtime_genesis_config() -> RuntimeGenesisConfig {
        Self::testnet_genesis()
    }

    /// Configure initial storage state for FRAME modules.
    fn testnet_genesis() -> RuntimeGenesisConfig {
        RuntimeGenesisConfig {
            system: SystemConfig {
                ..Default::default()
            },
            balances: BalancesConfig {
                balances: Self::endowed_accounts()
                    .iter()
                    .cloned()
                    .map(|k| (k, Self::initial_balance()))
                    .collect(),
            },
            aura: AuraConfig {
                authorities: Self::initial_authorities()
                    .iter()
                    .map(|x| x.0.clone())
                    .collect(),
            },
            grandpa: GrandpaConfig {
                authorities: Self::initial_authorities()
                    .iter()
                    .map(|x| (x.1.clone(), 1))
                    .collect(),
                ..Default::default()
            },
            sudo: SudoConfig {
                // Assign network admin rights.
                key: Some(Self::root_key()),
            },
            transaction_payment: Default::default(),
            ethereum: Default::default(),
            evm: EVMConfig {
                accounts: Self::get_evm_accounts(),
                ..Default::default()
            },
            dynamic_fee: Default::default(),
            base_fee: Default::default(),
            evm_chain_id: EVMChainIdConfig {
                chain_id: Self::evm_chain_id(),
                ..Default::default()
            },
        }
    }

    /// Returns the bootnodes.
    fn bootnodes() -> Vec<MultiaddrWithPeerId> {
        vec![]
    }

    /// Returns the telemetry endpoints.
    fn telemetry_endpoints() -> Option<TelemetryEndpoints> {
        Some(TelemetryEndpoints::new(vec![(String::from(DEFAULT_TELEMETRY_URL), 0)]).unwrap())
    }

    /// Returns the protocol ID.
    fn protocol_id() -> Option<&'static str> {
        None
    }

    /// Returns the fork ID.
    fn fork_id() -> Option<&'static str> {
        None
    }

    /// Builds the ChainSpec.
    fn build() -> ChainSpec {
        ChainSpec::from_genesis(
            Self::chain_name(),
            Self::chain_identifier(),
            Self::chain_type(),
            move || Self::runtime_genesis_config(),
            Self::bootnodes(),
            Self::telemetry_endpoints(),
            Self::protocol_id(),
            Self::fork_id(),
            Some(Self::chain_spec_prop()),
            Some(()),
            Self::wasm_binary(),
        )
    }

    /// Returns the EVM accounts.
    fn get_evm_accounts() -> BTreeMap<H160, fp_evm::GenesisAccount> {
        let accounts = Self::endowed_accounts();
        let mut map = BTreeMap::new();

        for account in accounts {
            let key = H160::from_slice(&account.0);

            let value = GenesisAccount {
                balance: U256::from(Self::initial_balance()),
                nonce: Default::default(),
                code: Default::default(),
                storage: Default::default(),
            };

            map.insert(key, value);
        }

        map
    }
}

/// Struct for development chain specification.
pub struct Dev;
/// Struct for local chain specification.
pub struct Local;
/// Struct for testnet chain specification.
pub struct Testnet;
/// Struct for mainnet chain specification.
pub struct Mainnet;
/// Struct for node chain specification.
pub struct NodeChainSpec<Env>(PhantomData<Env>);

impl CustomChainSpecProperties for NodeChainSpec<Dev> {
    fn token_symbol() -> &'static str {
        "BGLT"
    }

    fn token_decimals() -> u8 {
        18
    }

    fn evm_chain_id() -> u64 {
        6024
    }

    fn initial_balance() -> u128 {
        1_000_000 * BGLT
    }

    fn chain_name() -> &'static str {
        "ID chain development"
    }

    fn chain_identifier() -> &'static str {
        "dev"
    }

    fn chain_type() -> ChainType {
        ChainType::Development
    }
}

impl CustomChainSpecProperties for NodeChainSpec<Local> {
    fn token_symbol() -> &'static str {
        "BGLT"
    }

    fn token_decimals() -> u8 {
        18
    }

    fn evm_chain_id() -> u64 {
        6025
    }

    fn initial_balance() -> u128 {
        1_000_000 * BGLT
    }

    fn chain_name() -> &'static str {
        "ID chain local"
    }

    fn chain_identifier() -> &'static str {
        "local"
    }

    fn initial_authorities() -> Vec<(AuraId, GrandpaId)> {
        vec![
            authority_keys_from_seed("Alice"),
            authority_keys_from_seed("Bob"),
            authority_keys_from_seed("Charlie"),
            authority_keys_from_seed("Ferdie"),
        ]
    }

    fn chain_type() -> ChainType {
        ChainType::Local
    }
}

/// Struct for account-related utilities.
pub struct Account;

impl Account {
    /// Gets the ECDSA public key from a seed.
    pub fn get_from_seed_with_ecdsa(seed: &str) -> ecdsa::Public {
        ecdsa::Pair::from_string(&format!("//{}", seed), None)
            .expect("internal values are valid; qed")
            .public()
    }

    /// Truncates the first 20 bytes of the public key to generate an account ID.
    pub fn to_account_id_from_ecdsa(seed: ecdsa::Public) -> AccountId {
        let mut id = [0u8; 20];
        let seed_bytes: &[u8] = seed.as_ref();
        id.clone_from_slice(&seed_bytes[0..20]);

        fp_account::AccountId20(id)
    }

    /// Gets the EVM compatible account ID from a seed.
    pub fn get_evm_compatible_account_id_from_seed(seed: &str) -> AccountId {
        Self::to_account_id_from_ecdsa(Self::get_from_seed_with_ecdsa(seed))
    }

    /// Gets the public key from a seed.
    pub fn get_from_seed_with<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
        TPublic::Pair::from_string(&format!("//{}", seed), None)
            .expect("static values are valid; qed")
            .public()
    }
}

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<RuntimeGenesisConfig>;

/// Generates Aura and Grandpa authority keys from a seed.
pub fn authority_keys_from_seed(s: &str) -> (AuraId, GrandpaId) {
    (
        Account::get_from_seed_with::<AuraId>(s),
        Account::get_from_seed_with::<GrandpaId>(s),
    )
}
