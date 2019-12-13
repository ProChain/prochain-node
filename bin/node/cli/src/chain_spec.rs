// Copyright 2018-2019 Parity Technologies (UK) Ltd.
// This file is part of Substrate.

// Substrate is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Substrate is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Substrate.  If not, see <http://www.gnu.org/licenses/>.

//! Substrate chain configurations.

use chain_spec::ChainSpecExtension;
use primitives::{Pair, Public, crypto::UncheckedInto, sr25519};
use serde::{Serialize, Deserialize};
use node_runtime::{
	AuthorityDiscoveryConfig, BabeConfig, BalancesConfig, ContractsConfig, CouncilConfig, DemocracyConfig,
	GrandpaConfig, ImOnlineConfig, IndicesConfig, SessionConfig, SessionKeys, StakerStatus, StakingConfig, SudoConfig,
	SystemConfig, TechnicalCommitteeConfig, DidConfig, AdsConfig, WASM_BINARY,
};
use node_runtime::Block;
use node_runtime::constants::currency::*;
use sc_service;
use hex_literal::hex;
use std::fs::File;
use std::io::Read;
use sc_telemetry::TelemetryEndpoints;
use grandpa_primitives::{AuthorityId as GrandpaId};
use babe_primitives::{AuthorityId as BabeId};
use im_online::sr25519::{AuthorityId as ImOnlineId};
use authority_discovery_primitives::AuthorityId as AuthorityDiscoveryId;
use sp_runtime::{Perbill, traits::{Verify, IdentifyAccount}};
use hex::FromHex;

pub use node_primitives::{AccountId, Balance, Signature};
pub use node_runtime::GenesisConfig;

type AccountPublic = <Signature as Verify>::Signer;

const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";
const PRA_PROPERTIES: &str = r#"
		{
			"tokenDecimals": 15,
			"tokenSymbol": "PRA"
		}"#;

#[derive(Serialize, Deserialize)]
struct Allocation {
    balances: Vec<(String, String)>,
}
/// Node `ChainSpec` extensions.
///
/// Additional parameters for some Substrate core modules,
/// customizable from the chain spec.
#[derive(Default, Clone, Serialize, Deserialize, ChainSpecExtension)]
pub struct Extensions {
	/// Block numbers with known hashes.
	pub fork_blocks: client::ForkBlocks<Block>,
}

/// Specialized `ChainSpec`.
pub type ChainSpec = sc_service::ChainSpec<
	GenesisConfig,
	Extensions,
>;
/// Flaming Fir testnet generator
pub fn flaming_fir_config() -> Result<ChainSpec, String> {
	ChainSpec::from_json_bytes(&include_bytes!("../res/flaming-fir.json")[..])
}

fn session_keys(
	grandpa: GrandpaId,
	babe: BabeId,
	im_online: ImOnlineId,
	authority_discovery: AuthorityDiscoveryId,
) -> SessionKeys {
	SessionKeys { grandpa, babe, im_online, authority_discovery }
}

fn staging_testnet_config_genesis() -> GenesisConfig {
	// stash, controller, session-key
	// generated with secret:
	// for i in 1 2 3 4 ; do for j in stash controller; do subkey inspect "$secret"/fir/$j/$i; done; done
	// and
	// for i in 1 2 3 4 ; do for j in session; do subkey --ed25519 inspect "$secret"//fir//$j//$i; done; done

	let initial_authorities: Vec<(AccountId, AccountId, GrandpaId, BabeId, ImOnlineId, AuthorityDiscoveryId)> = vec![(
		// 5Fbsd6WXDGiLTxunqeK5BATNiocfCqu9bS1yArVjCgeBLkVy
		hex!["9c7a2ee14e565db0c69f78c7b4cd839fbf52b607d867e9e9c5a79042898a0d12"].into(),
		// 5EnCiV7wSHeNhjW3FSUwiJNkcc2SBkPLn5Nj93FmbLtBjQUq
		hex!["781ead1e2fa9ccb74b44c19d29cb2a7a4b5be3972927ae98cd3877523976a276"].into(),
		// 5Fb9ayurnxnaXj56CjmyQLBiadfRCqUbL2VWNbbe1nZU6wiC
		hex!["9becad03e6dcac03cee07edebca5475314861492cdfc96a2144a67bbe9699332"].unchecked_into(),
		// 5EZaeQ8djPcq9pheJUhgerXQZt9YaHnMJpiHMRhwQeinqUW8
		hex!["6e7e4eb42cbd2e0ab4cae8708ce5509580b8c04d11f6758dbf686d50fe9f9106"].unchecked_into(),
		// 5EZaeQ8djPcq9pheJUhgerXQZt9YaHnMJpiHMRhwQeinqUW8
		hex!["6e7e4eb42cbd2e0ab4cae8708ce5509580b8c04d11f6758dbf686d50fe9f9106"].unchecked_into(),
		// 5EZaeQ8djPcq9pheJUhgerXQZt9YaHnMJpiHMRhwQeinqUW8
		hex!["6e7e4eb42cbd2e0ab4cae8708ce5509580b8c04d11f6758dbf686d50fe9f9106"].unchecked_into(),
	),(
		// 5ERawXCzCWkjVq3xz1W5KGNtVx2VdefvZ62Bw1FEuZW4Vny2
		hex!["68655684472b743e456907b398d3a44c113f189e56d1bbfd55e889e295dfde78"].into(),
		// 5Gc4vr42hH1uDZc93Nayk5G7i687bAQdHHc9unLuyeawHipF
		hex!["c8dc79e36b29395413399edaec3e20fcca7205fb19776ed8ddb25d6f427ec40e"].into(),
		// 5EockCXN6YkiNCDjpqqnbcqd4ad35nU4RmA1ikM4YeRN4WcE
		hex!["7932cff431e748892fa48e10c63c17d30f80ca42e4de3921e641249cd7fa3c2f"].unchecked_into(),
		// 5DhLtiaQd1L1LU9jaNeeu9HJkP6eyg3BwXA7iNMzKm7qqruQ
		hex!["482dbd7297a39fa145c570552249c2ca9dd47e281f0c500c971b59c9dcdcd82e"].unchecked_into(),
		// 5DhLtiaQd1L1LU9jaNeeu9HJkP6eyg3BwXA7iNMzKm7qqruQ
		hex!["482dbd7297a39fa145c570552249c2ca9dd47e281f0c500c971b59c9dcdcd82e"].unchecked_into(),
		// 5DhLtiaQd1L1LU9jaNeeu9HJkP6eyg3BwXA7iNMzKm7qqruQ
		hex!["482dbd7297a39fa145c570552249c2ca9dd47e281f0c500c971b59c9dcdcd82e"].unchecked_into(),
	),(
		// 5DyVtKWPidondEu8iHZgi6Ffv9yrJJ1NDNLom3X9cTDi98qp
		hex!["547ff0ab649283a7ae01dbc2eb73932eba2fb09075e9485ff369082a2ff38d65"].into(),
		// 5FeD54vGVNpFX3PndHPXJ2MDakc462vBCD5mgtWRnWYCpZU9
		hex!["9e42241d7cd91d001773b0b616d523dd80e13c6c2cab860b1234ef1b9ffc1526"].into(),
		// 5E1jLYfLdUQKrFrtqoKgFrRvxM3oQPMbf6DfcsrugZZ5Bn8d
		hex!["5633b70b80a6c8bb16270f82cca6d56b27ed7b76c8fd5af2986a25a4788ce440"].unchecked_into(),
		// 5DhKqkHRkndJu8vq7pi2Q5S3DfftWJHGxbEUNH43b46qNspH
		hex!["482a3389a6cf42d8ed83888cfd920fec738ea30f97e44699ada7323f08c3380a"].unchecked_into(),
		// 5DhKqkHRkndJu8vq7pi2Q5S3DfftWJHGxbEUNH43b46qNspH
		hex!["482a3389a6cf42d8ed83888cfd920fec738ea30f97e44699ada7323f08c3380a"].unchecked_into(),
		// 5DhKqkHRkndJu8vq7pi2Q5S3DfftWJHGxbEUNH43b46qNspH
		hex!["482a3389a6cf42d8ed83888cfd920fec738ea30f97e44699ada7323f08c3380a"].unchecked_into(),
	),(
		// 5HYZnKWe5FVZQ33ZRJK1rG3WaLMztxWrrNDb1JRwaHHVWyP9
		hex!["f26cdb14b5aec7b2789fd5ca80f979cef3761897ae1f37ffb3e154cbcc1c2663"].into(),
		// 5EPQdAQ39WQNLCRjWsCk5jErsCitHiY5ZmjfWzzbXDoAoYbn
		hex!["66bc1e5d275da50b72b15de072a2468a5ad414919ca9054d2695767cf650012f"].into(),
		// 5DMa31Hd5u1dwoRKgC4uvqyrdK45RHv3CpwvpUC1EzuwDit4
		hex!["3919132b851ef0fd2dae42a7e734fe547af5a6b809006100f48944d7fae8e8ef"].unchecked_into(),
		// 5C4vDQxA8LTck2xJEy4Yg1hM9qjDt4LvTQaMo4Y8ne43aU6x
		hex!["00299981a2b92f878baaf5dbeba5c18d4e70f2a1fcd9c61b32ea18daf38f4378"].unchecked_into(),
		// 5C4vDQxA8LTck2xJEy4Yg1hM9qjDt4LvTQaMo4Y8ne43aU6x
		hex!["00299981a2b92f878baaf5dbeba5c18d4e70f2a1fcd9c61b32ea18daf38f4378"].unchecked_into(),
		// 5C4vDQxA8LTck2xJEy4Yg1hM9qjDt4LvTQaMo4Y8ne43aU6x
		hex!["00299981a2b92f878baaf5dbeba5c18d4e70f2a1fcd9c61b32ea18daf38f4378"].unchecked_into(),
	)];

	// generated with secret: subkey inspect "$secret"/fir
	let root_key: AccountId = hex![
		// 5Ff3iXP75ruzroPWRP2FYBHWnmGGBSb63857BgnzCoXNxfPo
		"9ee5e5bdc0ec239eb164f865ecc345ce4c88e76ee002e0f7e318097347471809"
	].into();

	let endowed_accounts: Vec<AccountId> = vec![root_key.clone()];

	testnet_genesis(
		initial_authorities,
		root_key,
		Some(endowed_accounts),
		false,
	)
}

/// Staging testnet config.
pub fn staging_testnet_config() -> ChainSpec {
	let boot_nodes = vec![];
	ChainSpec::from_genesis(
		"Staging Testnet",
		"staging_testnet",
		staging_testnet_config_genesis,
		boot_nodes,
		Some(TelemetryEndpoints::new(vec![(STAGING_TELEMETRY_URL.to_string(), 0)])),
		None,
		None,
		Default::default(),
	)
}

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate stash, controller and session key from seed
pub fn get_authority_keys_from_seed(seed: &str) -> (
	AccountId,
	AccountId,
	GrandpaId,
	BabeId,
	ImOnlineId,
	AuthorityDiscoveryId,
) {
	(
		get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", seed)),
		get_account_id_from_seed::<sr25519::Public>(seed),
		get_from_seed::<GrandpaId>(seed),
		get_from_seed::<BabeId>(seed),
		get_from_seed::<ImOnlineId>(seed),
		get_from_seed::<AuthorityDiscoveryId>(seed),
	)
}

/// Helper function to create GenesisConfig for testing
pub fn testnet_genesis(
	initial_authorities: Vec<(AccountId, AccountId, GrandpaId, BabeId, ImOnlineId, AuthorityDiscoveryId)>,
	root_key: AccountId,
	endowed_accounts: Option<Vec<AccountId>>,
	enable_println: bool,
) -> GenesisConfig {
	let endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(|| {
		vec![
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			get_account_id_from_seed::<sr25519::Public>("Bob"),
			get_account_id_from_seed::<sr25519::Public>("Charlie"),
			get_account_id_from_seed::<sr25519::Public>("Dave"),
			get_account_id_from_seed::<sr25519::Public>("Eve"),
			get_account_id_from_seed::<sr25519::Public>("Ferdie"),
			get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
			get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
			get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
			get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
			get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
			get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
		]
	});

	const ENDOWMENT: Balance = 100_000_000 * DOLLARS;
	const STASH: Balance = 10_000 * DOLLARS;

	GenesisConfig {
		system: Some(SystemConfig {
			code: WASM_BINARY.to_vec(),
			changes_trie_config: Default::default(),
		}),
		balances: Some(BalancesConfig {
			balances: endowed_accounts.iter().cloned()
				.map(|k| (k, ENDOWMENT))
				.chain(initial_authorities.iter().map(|x| (x.0.clone(), STASH)))
				.collect(),
			vesting: vec![],
		}),
		indices: Some(IndicesConfig {
			ids: endowed_accounts.iter().cloned()
				.chain(initial_authorities.iter().map(|x| x.0.clone()))
				.collect::<Vec<_>>(),
		}),
		session: Some(SessionConfig {
			keys: initial_authorities.iter().map(|x| {
				(x.0.clone(), session_keys(x.2.clone(), x.3.clone(), x.4.clone(), x.5.clone()))
			}).collect::<Vec<_>>(),
		}),
		staking: Some(StakingConfig {
			current_era: 0,
			validator_count: initial_authorities.len() as u32 * 2,
			minimum_validator_count: initial_authorities.len() as u32,
			stakers: initial_authorities.iter().map(|x| {
				(x.0.clone(), x.1.clone(), STASH, StakerStatus::Validator)
			}).collect(),
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			slash_reward_fraction: Perbill::from_percent(10),
			.. Default::default()
		}),
		democracy: Some(DemocracyConfig::default()),
		collective_Instance1: Some(CouncilConfig {
			members: endowed_accounts.iter().cloned()
				.collect::<Vec<_>>()[..].to_vec(),
			phantom: Default::default(),
		}),
		collective_Instance2: Some(TechnicalCommitteeConfig {
			members: endowed_accounts.iter().cloned()
				.collect::<Vec<_>>()[..].to_vec(),
			phantom: Default::default(),
		}),
		contracts: Some(ContractsConfig {
			current_schedule: contracts::Schedule {
				enable_println, // this should only be enabled on development chains
				..Default::default()
			},
			gas_price: 1 * MILLICENTS,
		}),
		sudo: Some(SudoConfig {
			key: root_key,
		}),
		babe: Some(BabeConfig {
			authorities: vec![],
		}),
		im_online: Some(ImOnlineConfig {
			keys: vec![],
		}),
		authority_discovery: Some(AuthorityDiscoveryConfig {
			keys: vec![],
		}),
		grandpa: Some(GrandpaConfig {
			authorities: vec![],
		}),
		membership_Instance1: Some(Default::default()),
		treasury: Some(Default::default()),
		did: Some(DidConfig {
			genesis_account: hex!["22df4b685df33f070ae6e5ee27f745de078adff099d3a803ec67afe1168acd4f"].into(),
			min_deposit: 50 * DOLLARS,
			base_quota: 250,
			fee_to_previous: 25 * DOLLARS,
		}),
		ads: Some(AdsConfig {
			contract: hex!["22df4b685df33f070ae6e5ee27f745de078adff099d3a803ec67afe1168acd4f"].into(),
			min_deposit: 500 * DOLLARS,
		}),
	}
}

fn development_config_genesis() -> GenesisConfig {
	testnet_genesis(
		vec![
			get_authority_keys_from_seed("Alice"),
		],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		None,
		true,
	)
}

/// Development config (single validator Alice)
pub fn development_config() -> ChainSpec {
	let properties = serde_json::from_str(PRA_PROPERTIES).unwrap();
	ChainSpec::from_genesis(
		"Development",
		"dev",
		development_config_genesis,
		vec![],
		None,
		None,
		properties,
		Default::default(),
	)
}

fn local_testnet_genesis() -> GenesisConfig {
	testnet_genesis(
		vec![
			get_authority_keys_from_seed("Alice"),
			get_authority_keys_from_seed("Bob"),
		],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		None,
		false,
	)
}

/// Local testnet config (multivalidator Alice + Bob)
pub fn local_testnet_config() -> ChainSpec {
	ChainSpec::from_genesis(
		"Local Testnet",
		"local_testnet",
		local_testnet_genesis,
		vec![],
		None,
		None,
		None,
		Default::default(),
	)
}

fn prochain_testnet_genesis() -> GenesisConfig {
		let initial_authorities: Vec<(AccountId, AccountId, GrandpaId, BabeId, ImOnlineId, AuthorityDiscoveryId)> = vec![(
			// 5GTDPKDnqavJMa9Wsqp8ospmjs3riV6zg8obQNLPqn7c3wh5
			hex!["c21bbbee5958ccc3be18979a75c229dfb8ad9319218eddc71b0989c796408b13"].into(),
			// 5CSMAWVYBoHpMYTjxS2AWYEo74ntuHNdQFuxzuD1Cz37ffcm
			hex!["1081bdf73aade46b14187607f1ff944876cc886282e2af9aab0c6a5b5ddb6d02"].into(),
			// 5FaXqX3rHbsE31FoJAZVE1w7jiRdrURL7EBbQ6SLY68nQ4W9
			hex!["9b74552f41e4662db1193d588f752cfb4d7d4947eac64e9c71bf6a004cda2901"].unchecked_into(),
			// 5FmzqTxeUqaj1gZeUUo8KUx61AY3NmJgHYZD686r3J6DmHiL
			hex!["a432eb5161754017b94b7ec93d6d45b4ddc1c3137d0bef694c9211a146b95969"].unchecked_into(),
			// 5HZAZSLmQj1WvSP1uM3LGeFqimYjkamsNsLmNedDmfbBmsL2
			hex!["f2e1edb9244c3dc5307ae45c76aab1c2f3524ab6aa0d03152a80c0e7b70cc902"].unchecked_into(),
			// 5CDWPLg2NMYhSPn1UVcuAR2Fzn948FJvgxsuLWrCLGexjDBd
			hex!["06b697db1eb33fe300e9b292213da1e659f5e27f6f4cf28bb9b141bfc3af6079"].unchecked_into(),
		),(
			// 5ECkqhw4dCCwX6zzanbjNCgA9VxMeK6qR7snfrWefbumLBrQ
			hex!["5e9c79234b5e55348fc60f38b28c2cc60d8bb4bd2862eae2179a05ec39e62658"].into(),
			// 5H4F6CRnkUaMGSYckLXMViNTECpa4pC8KNAKW4X9qjKF73CC
			hex!["dcd30ff89083ced6197c950ab9409989ebddbee39ebe3e771ad6cd352da1d178"].into(),
			// 5CxQtKD3zcrqqGBRVN6jJqNeRWHC9WtF5ph6vN89r7EjV82R
			hex!["276fa1242eda3dfb9bdabd3a3c87b07c7f52ef74deec6bb980867a99ef143db7"].unchecked_into(),
			// 5DXCactxvDJtXyR4auEd3WDc2Z1ZgQWETUbCDm2o476VBX1h
			hex!["40714e20c9ed2915de752e2a0a9d952be406afbd68820ec292c3e8016b592e62"].unchecked_into(),
			// 5GhERWzdYM5qwQfWduULHPRKwgB9MGtmNXyresnbX4vWWv11
			hex!["cccca9f7232c7ee9dc60dcc301425dff18087b79963020166b4b6cd432bb3075"].unchecked_into(),
			// 5EeDYacpVo7zvjno5VGiHJocc6gvHFj3envE4enALRLxayrN
			hex!["7207812127e3c66678df599e100b93426b2a28b2bf85fb57685351e00ab2f162"].unchecked_into(),
		),(
			// 5HLCVgpCPQrSXasbzHahRR9HuT1uEj2NQmMiEC4fS3J15azc
			hex!["e8fe40d68fc1efe504b9a709bd8591e4402f3162b8297155708e34a46cd7272d"].into(),
			// 5HYyYS4tVA5QCH1WfyfWfSrZpTnEoCq7V7MsbjgaGYU5FK4C
			hex!["f2bcd74b3e1775d68c5aadd804500b312e05a434ec3ad4df2b1db91a666f7601"].into(),
			// 5CYgmnEBHHxT5BdZxYqYqBgSDE1SvCY7SbT4NeLSNZX1Dm7R
			hex!["155738a81c5eb4040922493a4f834b7b935013061af1d1ea85264addc43bf84f"].unchecked_into(),
			// 5EHYQX755SfGXKtbaxGZARwsL2D5d1nxt3GfjshTNGypSfe5
			hex!["6242d7c10c7887734f367c6ab2f4bbc5ef7bde7a5aff4fbbbb35cbdbc7898231"].unchecked_into(),
			// 5Eh1BeG8xNhk6NmmK2VYVgvxnMdvvfvMQomUsN3XRktaH5v6
			hex!["7427a13d0757415eeadeffc33c490a402f3bf46a6dfba2f6a1145ae3cd747c6d"].unchecked_into(),
			// 5Gj1Q4eVG8tdEVyGbrLBK6mdp9df1wmsxxHMR4nn7vA1n8hw
			hex!["ce274ef545a0ca52952d7e3043431afc1007ba4b6a440e6b60517cf817a3c03e"].unchecked_into(),
		),(
			// 5DSVnbWsmju4raE6nALKAdg6iiJau87vU6cvpwexVZ3Pr2f2
			hex!["3cdb0017aef46c82411926506f0335157cb3b706cd03f1b65c99bdf7b0288444"].into(),
			// 5Cetf6wLBM55RxmDqrqLnLMCsaNgJRgEs5iMp945nhKZDbPz
			hex!["1a12b3a84fba82e444b51586f62ec7ed41b8ce09b6a7bd4639dd4e4c6c782e33"].into(),
			// 5DnEMQX5BgSJZ235UQidEDwRvt4xGjPUE8M3hsFiRpXakipz
			hex!["4be7f3b31f770d59d29e38d19592d65ec3f1ea72c62b35df2094d093ba7b0076"].unchecked_into(),
			// 5Dvj8ZbJc8eqJpqKLf4qLvc463iXcgwK7zbdPcuqK69n73U3
			hex!["5262cade2d3e92a6f164c8ef93f9e5c1570761e92b5e98e31f721cf43cb9913b"].unchecked_into(),
			// 5G3ix9U3gdMFbTQgLrvwuKyya1ok9FkECvri21cEe56968Rm
			hex!["b0315f660d8a57f6833b9f6403ae3c2eb4a015fc0431e8f1ff24d0c65fefaf52"].unchecked_into(),
			// 5G1BHvm3h9D3PriPiqpbQFjR9p7JPrQtwknS7VevpKqGS8ym
			hex!["ae404eede3214ba00d5a34964820d5b6da578b8d0199527b20c27d2e0e04de29"].unchecked_into(),
		)];

		// generated with secret: subkey inspect "$secret"/fir
		let root_key: AccountId = hex![
			// 5CrRpNbQBTiBmTjpUgJ6mH9YRmopVweLsjffVz7muskYEo2r
			"22df4b685df33f070ae6e5ee27f745de078adff099d3a803ec67afe1168acd4f"
		].into();

		let endowed_accounts: Vec<AccountId> = vec![root_key.clone()];
		let intial_allocation_json = get_intial_allocation().unwrap();
		let intial_allocation = intial_allocation_json.0;
		let intial_total = intial_allocation_json.1;

		const STASH: Balance = 10_000 * DOLLARS;
		let total_stash: Balance = 10_000 * initial_authorities.len() as u128 * DOLLARS;
		let endowed_amount: Balance = 100_000_000 * DOLLARS - intial_total - total_stash;

		GenesisConfig {
			system: Some(SystemConfig {
				code: WASM_BINARY.to_vec(),
				changes_trie_config: Default::default(),
			}),
			balances: Some(BalancesConfig {
				balances: endowed_accounts.iter().cloned()
					.map(|k| (k, endowed_amount))
					.chain(initial_authorities.iter().map(|x| (x.0.clone(), STASH)))
					.chain(intial_allocation.iter().map(|x| (x.0.clone(), x.1.clone())))
					.collect(),
				vesting: vec![],
			}),
			indices: Some(IndicesConfig {
				ids: endowed_accounts.iter().cloned()
					.chain(initial_authorities.iter().map(|x| x.0.clone()))
					.chain(intial_allocation.iter().map(|x| x.0.clone()))
					.collect::<Vec<_>>(),
			}),
			session: Some(SessionConfig {
				keys: initial_authorities.iter().map(|x| {
					(x.0.clone(), session_keys(x.2.clone(), x.3.clone(), x.4.clone(), x.5.clone()))
				}).collect::<Vec<_>>(),
			}),
			staking: Some(StakingConfig {
				current_era: 0,
				validator_count: initial_authorities.len() as u32 * 2,
				minimum_validator_count: initial_authorities.len() as u32,
				stakers: initial_authorities.iter().map(|x| {
					(x.0.clone(), x.1.clone(), STASH, StakerStatus::Validator)
				}).collect(),
				invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
				slash_reward_fraction: Perbill::from_percent(10),
				.. Default::default()
			}),
			democracy: Some(DemocracyConfig::default()),
			collective_Instance1: Some(CouncilConfig {
				members: initial_authorities.iter().map(|x| x.0.clone()).collect(),
				phantom: Default::default(),
			}),
			collective_Instance2: Some(TechnicalCommitteeConfig {
				members: initial_authorities.iter().map(|x| x.0.clone()).collect(),
				phantom: Default::default(),
			}),
			contracts: Some(ContractsConfig {
				current_schedule: contracts::Schedule {
					enable_println: false, // this should only be enabled on development chains
					..Default::default()
				},
				gas_price: 1 * MILLICENTS,
			}),
			sudo: Some(SudoConfig {
				key: root_key,
			}),
			babe: Some(BabeConfig {
				authorities: vec![],
			}),
			im_online: Some(ImOnlineConfig {
				keys: vec![],
			}),
			authority_discovery: Some(AuthorityDiscoveryConfig {
				keys: vec![],
			}),
			grandpa: Some(GrandpaConfig {
				authorities: vec![],
			}),
			membership_Instance1: Some(Default::default()),
			treasury: Some(Default::default()),
			did: Some(DidConfig {
				genesis_account: hex!["22df4b685df33f070ae6e5ee27f745de078adff099d3a803ec67afe1168acd4f"].into(),
				min_deposit: 50 * DOLLARS,
				base_quota: 250,
				fee_to_previous: 25 * DOLLARS,
			}),
			ads: Some(AdsConfig {
				contract: hex!["22df4b685df33f070ae6e5ee27f745de078adff099d3a803ec67afe1168acd4f"].into(),
				min_deposit: 500 * DOLLARS,
			}),
		}
	}

	/// prochain testnet config
	pub fn prochain_testnet_config() -> ChainSpec {
		// let boot_nodes = vec![
		// 	"/ip4/39.106.220.238/tcp/30333/p2p/QmdcynTigKkriECrcE48DW5hdd45cmnQwiZfBGW6AoQ8EA".to_string(),
		// 	"/ip4/123.206.52.244/tcp/30333/p2p/QmVwKhj4XjcjTfBCUpN7QUHpWvniwPVQvoSGsgJn8Y5BAH".to_string(),
		// ];
		let boot_nodes = vec![];
		let properties = serde_json::from_str(PRA_PROPERTIES).unwrap();
		ChainSpec::from_genesis(
			"Prochain Testnet",
			"prochain_testnet",
			prochain_testnet_genesis,
			boot_nodes,
			Some(TelemetryEndpoints::new(vec![(STAGING_TELEMETRY_URL.to_string(), 0)])),
			None,
			properties,
			Default::default(),
		)
	}

	// Give each intial participant the allocation,
	fn get_intial_allocation() -> Result<(Vec<(AccountId, Balance)>, Balance), String> {
		let mut file = File::open("intial_drop.json").expect("Unable to open");
		let mut data = String::new();
		file.read_to_string(&mut data).unwrap();

		let json: Allocation = serde_json::from_str(&data).unwrap();
		let balances_json = json.balances;

		let balances: Vec<(AccountId, Balance)> = balances_json.clone().into_iter().map(|e| {
			return (
				<[u8; 32]>::from_hex(e.0).unwrap().into(),
				e.1.to_string().parse::<Balance>().unwrap(),
			);
		}).collect();

		let total: Balance = balances_json.into_iter().map(|e| {
			e.1.to_string().parse::<Balance>().unwrap()
		}).sum();
		Ok((balances, total))
	}

#[cfg(test)]
pub(crate) mod tests {
	use super::*;
	use crate::service::new_full;
	use sc_service::Roles;
	use service_test;

	fn local_testnet_genesis_instant_single() -> GenesisConfig {
		testnet_genesis(
			vec![
				get_authority_keys_from_seed("Alice"),
			],
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			None,
			false,
		)
	}

	/// Local testnet config (single validator - Alice)
	pub fn integration_test_config_with_single_authority() -> ChainSpec {
		ChainSpec::from_genesis(
			"Integration Test",
			"test",
			local_testnet_genesis_instant_single,
			vec![],
			None,
			None,
			None,
			Default::default(),
		)
	}

	/// Local testnet config (multivalidator Alice + Bob)
	pub fn integration_test_config_with_two_authorities() -> ChainSpec {
		ChainSpec::from_genesis(
			"Integration Test",
			"test",
			local_testnet_genesis,
			vec![],
			None,
			None,
			None,
			Default::default(),
		)
	}

	#[test]
	#[ignore]
	fn test_connectivity() {
		service_test::connectivity(
			integration_test_config_with_two_authorities(),
			|config| new_full(config),
			|mut config| {
				// light nodes are unsupported
				config.roles = Roles::FULL;
				new_full(config)
			},
			true,
		);
	}
}
