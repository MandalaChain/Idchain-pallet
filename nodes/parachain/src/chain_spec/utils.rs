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

use runtime_common::{AccountId, AccountPublic};
use sc_service::Properties;
use sp_core::{Pair, Public};
use sp_runtime::traits::IdentifyAccount;

use crate::chain_spec::{self, ParachainMainnetRuntime, ParachainRuntime, ParachainTestnetRuntime};

/// Helper function to generate an account ID from seed
pub(crate) fn get_account_id_from_secret<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_public_key_from_secret::<TPublic>(seed)).into_account()
}

/// Helper function to generate a crypto pair from seed
pub(crate) fn get_public_key_from_secret<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

pub(crate) fn get_properties(symbol: &str, decimals: u32, ss58format: u32) -> Properties {
	Properties::from_iter([
		("tokenSymbol".into(), symbol.into()),
		("tokenDecimals".into(), decimals.into()),
		("ss58Format".into(), ss58format.into()),
	])
}

pub(crate) fn load_spec(id: &str) -> Result<Box<dyn sc_service::ChainSpec>, String> {
	let runtime = id.parse::<ParachainRuntime>()?;

	match runtime {
		ParachainRuntime::ParachainTestnet(pr) => match pr {
			ParachainTestnetRuntime::Dev => Ok(Box::new(chain_spec::testnet::dev::generate_chain_spec("rococo_local"))),
			ParachainTestnetRuntime::New => Ok(Box::new(chain_spec::testnet::new::generate_chain_spec())),
			ParachainTestnetRuntime::ParachainTestnet => Ok(Box::new(chain_spec::testnet::ChainSpec::from_json_bytes(
				include_bytes!("../../../../chainspecs/parachain-testnet/parachain-testnet-paseo.json").as_slice(),
			)?)),
			ParachainTestnetRuntime::ParachainTestnetStg => {
				Ok(Box::new(chain_spec::testnet::ChainSpec::from_json_bytes(
					include_bytes!("../../../../chainspecs/parachain-testnet-stg/parachain-testnet-stg.json")
						.as_slice(),
				)?))
			}
			ParachainTestnetRuntime::Standalone => Ok(Box::new(chain_spec::testnet::ChainSpec::from_json_bytes(
				include_bytes!("../../../../chainspecs/standalone/standalone-testnet.json").as_slice(),
			)?)),
			ParachainTestnetRuntime::StandaloneNew => Ok(Box::new(chain_spec::standalone::new::generate_chain_spec())),
			ParachainTestnetRuntime::Other(s) => Ok(Box::new(chain_spec::testnet::load_chain_spec(s.as_str())?)),
		},
		ParachainRuntime::ParachainMainnet(pmr) => match pmr {
			ParachainMainnetRuntime::Dev => Ok(Box::new(chain_spec::mainnet::dev::generate_chain_spec("rococo_local"))),
			ParachainMainnetRuntime::New => Ok(Box::new(chain_spec::mainnet::new::generate_chain_spec())),
			ParachainMainnetRuntime::ParachainMainnet => Ok(Box::new(chain_spec::mainnet::ChainSpec::from_json_bytes(
				include_bytes!("../../../../chainspecs/parachain-mainnet/parachain-mainnet.json").as_slice(),
			)?)),
			ParachainMainnetRuntime::Other(s) => Ok(Box::new(chain_spec::mainnet::load_chain_spec(s.as_str())?)),
		},
	}
}
