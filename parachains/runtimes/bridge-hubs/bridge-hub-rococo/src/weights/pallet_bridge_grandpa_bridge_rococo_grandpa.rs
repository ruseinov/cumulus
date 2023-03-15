// Copyright 2021 Parity Technologies (UK) Ltd.
// This file is part of Cumulus.

// Cumulus is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Cumulus is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Cumulus.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for `pallet_bridge_grandpa`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-03-15, STEPS: `10`, REPEAT: `1`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bkontur-ThinkPad-P14s-Gen-2i`, CPU: `11th Gen Intel(R) Core(TM) i7-1185G7 @ 3.00GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("bridge-hub-rococo-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot-parachain
// benchmark
// pallet
// --steps=10
// --repeat=1
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --json-file=./bench.json
// --header=./file_header.txt
// --chain=bridge-hub-rococo-dev
// --pallet=pallet_bridge_grandpa
// --output=./parachains/runtimes/bridge-hubs/bridge-hub-rococo/src/weights

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_bridge_grandpa`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_bridge_grandpa::WeightInfo for WeightInfo<T> {
	/// Storage: BridgeRococoGrandpa PalletOperatingMode (r:1 w:0)
	/// Proof: BridgeRococoGrandpa PalletOperatingMode (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	/// Storage: BridgeRococoGrandpa RequestCount (r:1 w:1)
	/// Proof: BridgeRococoGrandpa RequestCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: BridgeRococoGrandpa BestFinalized (r:1 w:1)
	/// Proof: BridgeRococoGrandpa BestFinalized (max_values: Some(1), max_size: Some(36), added: 531, mode: MaxEncodedLen)
	/// Storage: BridgeRococoGrandpa CurrentAuthoritySet (r:1 w:0)
	/// Proof: BridgeRococoGrandpa CurrentAuthoritySet (max_values: Some(1), max_size: Some(50250), added: 50745, mode: MaxEncodedLen)
	/// Storage: BridgeRococoGrandpa ImportedHashesPointer (r:1 w:1)
	/// Proof: BridgeRococoGrandpa ImportedHashesPointer (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: BridgeRococoGrandpa ImportedHashes (r:1 w:1)
	/// Proof: BridgeRococoGrandpa ImportedHashes (max_values: Some(1024), max_size: Some(36), added: 1521, mode: MaxEncodedLen)
	/// Storage: BridgeRococoGrandpa ImportedHeaders (r:0 w:2)
	/// Proof: BridgeRococoGrandpa ImportedHeaders (max_values: Some(1024), max_size: Some(68), added: 1553, mode: MaxEncodedLen)
	/// The range of component `p` is `[1, 838]`.
	/// The range of component `v` is `[50, 100]`.
	/// The range of component `p` is `[1, 838]`.
	/// The range of component `v` is `[50, 100]`.
	fn submit_finality_proof(p: u32, v: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `295 + p * (60 ±0)`
		//  Estimated: `60231`
		// Minimum execution time: 616_404_000 picoseconds.
		Weight::from_parts(616_404_000, 0)
			.saturating_add(Weight::from_parts(0, 60231))
			// Standard Error: 10_615_697
			.saturating_add(Weight::from_parts(107_372_374, 0).saturating_mul(p.into()))
			// Standard Error: 81_776_733
			.saturating_add(Weight::from_parts(97_333_232, 0).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(6))
	}
}
