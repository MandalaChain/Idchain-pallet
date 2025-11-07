// KILT Blockchain – https://botlabs.org
// Copyright (C) 2019-2024 BOTLabs GmbH

// The KILT Blockchain is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The KILT Blockchain is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// If you feel like getting in touch with us, you can do so at info@botlabs.org

//! KILT chain specification

use std::str::FromStr;

use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use serde::{Deserialize, Serialize};

pub(crate) use utils::load_spec;

pub(crate) mod mainnet;
pub(crate) mod standalone;
pub(crate) mod testnet;
pub(crate) mod utils;

const IDCHAIN_PARA_ID: u32 = 2_086;
const IDCHAIN_STANDALONE_ID: u32 = 4504;

/// The extensions for the `ChainSpec`.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, ChainSpecGroup, ChainSpecExtension)]
#[serde(deny_unknown_fields)]
pub struct Extensions {
	/// The relay chain of the Parachain.
	pub relay_chain: String,
	/// The id of the Parachain.
	pub para_id: u32,
}

impl Extensions {
	/// Try to get the extension from the given `ChainSpec`.
	pub(crate) fn try_get(chain_spec: &dyn sc_service::ChainSpec) -> Option<&Self> {
		sc_chain_spec::get_extension(chain_spec.extensions())
	}
}

pub(crate) enum ParachainTestnetRuntime {
	Dev,
	ParachainTestnet,
	ParachainTestnetStg,
	New,
	Standalone,
	StandaloneNew,
	Other(String),
}

impl std::fmt::Display for ParachainTestnetRuntime {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Dev => write!(f, "dev"),
			Self::ParachainTestnet => write!(f, "parachain-testnet"),
			Self::ParachainTestnetStg => write!(f, "parachain-testnet-stg"),
			Self::New => write!(f, "new"),
			Self::Standalone => write!(f, "standalone"),
			Self::StandaloneNew => write!(f, "standalone-new"),
			Self::Other(path) => write!(f, "other -> {path}"),
		}
	}
}

pub(crate) enum ParachainMainnetRuntime {
	Dev,
	ParachainMainnet,
	New,
	Other(String),
}

impl std::fmt::Display for ParachainMainnetRuntime {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Dev => write!(f, "dev"),
			Self::ParachainMainnet => write!(f, "parachain-mainnet"),
			Self::New => write!(f, "new"),
			Self::Other(path) => write!(f, "other -> {path}"),
		}
	}
}

pub(crate) enum ParachainRuntime {
	ParachainTestnet(ParachainTestnetRuntime),
	ParachainMainnet(ParachainMainnetRuntime),
}

impl std::fmt::Display for ParachainRuntime {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::ParachainTestnet(p) => write!(f, "parachain-testnet ({p})"),
			Self::ParachainMainnet(s) => write!(f, "parachain-mainnet ({s})"),
		}
	}
}

impl FromStr for ParachainRuntime {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			// Parachain-Testnet development
			"dev" | "parachain-testnet-dev" => Ok(Self::ParachainTestnet(ParachainTestnetRuntime::Dev)),
			// New blank Parachain-Testnet chainspec
			"parachain-testnet-new" => Ok(Self::ParachainTestnet(ParachainTestnetRuntime::New)),
			// Parachain-Testnet chainspec
			"parachain-testnet" => Ok(Self::ParachainTestnet(ParachainTestnetRuntime::ParachainTestnet)),
			// Staging Parachain-Testnet chainspec
			"parachain-testnet-stg" => Ok(Self::ParachainTestnet(ParachainTestnetRuntime::ParachainTestnetStg)),
			// Standalone chain chainspec
			"standalone" => Ok(Self::ParachainTestnet(ParachainTestnetRuntime::Standalone)),
			"standalone-new" => Ok(Self::ParachainTestnet(ParachainTestnetRuntime::StandaloneNew)),
			// Any other Parachain-Testnet-based chainspec
			s if s.contains("parachain-testnet") => {
				Ok(Self::ParachainTestnet(ParachainTestnetRuntime::Other(s.to_string())))
			}

			// Parachain-Mainnet development
			"parachain-mainnet-dev" => Ok(Self::ParachainMainnet(ParachainMainnetRuntime::Dev)),
			// New blank Parachain-Mainnet chainspec
			"parachain-mainnet-new" => Ok(Self::ParachainMainnet(ParachainMainnetRuntime::New)),
			// Parachain-Mainnet chainspec
			"parachain-mainnet" => Ok(Self::ParachainMainnet(ParachainMainnetRuntime::ParachainMainnet)),
			// Any other Parachain-Mainnet-based chainspec
			s if s.contains("parachain-mainnet") => {
				Ok(Self::ParachainMainnet(ParachainMainnetRuntime::Other(s.to_string())))
			}

			// Instead of panicking, we use the Parachain-Testnet runtime, since we don't expect Parachain-Mainnet to
			// ever be used in this way
			path => Ok(Self::ParachainTestnet(ParachainTestnetRuntime::Other(path.to_string()))),
		}
	}
}
