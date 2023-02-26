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

//! Autogenerated weights for `pallet_session`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-25, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `runner-9fxy16xz-project-238-concurrent-0`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("statemint-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot-parachain
// benchmark
// pallet
// --chain=statemint-dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=pallet_session
// --no-storage-info
// --no-median-slopes
// --no-min-squares
// --extrinsic=*
// --steps=50
// --repeat=20
// --json
// --header=./file_header.txt
// --output=./parachains/runtimes/assets/statemint/src/weights/pallet_session.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_session`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_session::WeightInfo for WeightInfo<T> {
	/// Storage: Session NextKeys (r:1 w:1)
	/// Proof Skipped: Session NextKeys (max_values: None, max_size: None, mode: Measured)
	/// Storage: Session KeyOwner (r:1 w:1)
	/// Proof Skipped: Session KeyOwner (max_values: None, max_size: None, mode: Measured)
	fn set_keys() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `270`
		//  Estimated: `5490`
		// Minimum execution time: 15_510 nanoseconds.
		Weight::from_ref_time(16_215_000)
			.saturating_add(Weight::from_proof_size(5490))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Session NextKeys (r:1 w:1)
	/// Proof Skipped: Session NextKeys (max_values: None, max_size: None, mode: Measured)
	/// Storage: Session KeyOwner (r:0 w:1)
	/// Proof Skipped: Session KeyOwner (max_values: None, max_size: None, mode: Measured)
	fn purge_keys() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `242`
		//  Estimated: `2959`
		// Minimum execution time: 11_819 nanoseconds.
		Weight::from_ref_time(12_321_000)
			.saturating_add(Weight::from_proof_size(2959))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}
