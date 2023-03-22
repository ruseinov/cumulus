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

//! Autogenerated weights for `pallet_uniques`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-03-22, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bm5`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("statemint-dev"), DB CACHE: 1024

// Executed Command:
// ./artifacts/polkadot-parachain
// benchmark
// pallet
// --chain=statemint-dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=pallet_uniques
// --extrinsic=*
// --steps=50
// --repeat=20
// --json
// --header=./file_header.txt
// --output=./parachains/runtimes/assets/statemint/src/weights/pallet_uniques.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_uniques`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_uniques::WeightInfo for WeightInfo<T> {
	/// Storage: Uniques Class (r:1 w:1)
	/// Proof: Uniques Class (max_values: None, max_size: Some(178), added: 2653, mode: MaxEncodedLen)
	/// Storage: Uniques ClassAccount (r:0 w:1)
	/// Proof: Uniques ClassAccount (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	fn create() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `145`
		//  Estimated: `3643`
		// Minimum execution time: 26_373_000 picoseconds.
		Weight::from_parts(26_988_000, 0)
			.saturating_add(Weight::from_parts(0, 3643))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Uniques Class (r:1 w:1)
	/// Proof: Uniques Class (max_values: None, max_size: Some(178), added: 2653, mode: MaxEncodedLen)
	/// Storage: Uniques ClassAccount (r:0 w:1)
	/// Proof: Uniques ClassAccount (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	fn force_create() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42`
		//  Estimated: `3643`
		// Minimum execution time: 14_777_000 picoseconds.
		Weight::from_parts(15_299_000, 0)
			.saturating_add(Weight::from_parts(0, 3643))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Uniques Class (r:1 w:1)
	/// Proof: Uniques Class (max_values: None, max_size: Some(178), added: 2653, mode: MaxEncodedLen)
	/// Storage: Uniques Asset (r:1001 w:1000)
	/// Proof: Uniques Asset (max_values: None, max_size: Some(122), added: 2597, mode: MaxEncodedLen)
	/// Storage: Uniques InstanceMetadataOf (r:1000 w:1000)
	/// Proof: Uniques InstanceMetadataOf (max_values: None, max_size: Some(187), added: 2662, mode: MaxEncodedLen)
	/// Storage: Uniques Attribute (r:1000 w:1000)
	/// Proof: Uniques Attribute (max_values: None, max_size: Some(172), added: 2647, mode: MaxEncodedLen)
	/// Storage: Uniques ClassAccount (r:0 w:1)
	/// Proof: Uniques ClassAccount (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	/// Storage: Uniques ClassMetadataOf (r:0 w:1)
	/// Proof: Uniques ClassMetadataOf (max_values: None, max_size: Some(167), added: 2642, mode: MaxEncodedLen)
	/// Storage: Uniques Account (r:0 w:1000)
	/// Proof: Uniques Account (max_values: None, max_size: Some(88), added: 2563, mode: MaxEncodedLen)
	/// Storage: Uniques CollectionMaxSupply (r:0 w:1)
	/// Proof: Uniques CollectionMaxSupply (max_values: None, max_size: Some(24), added: 2499, mode: MaxEncodedLen)
	/// The range of component `n` is `[0, 1000]`.
	/// The range of component `m` is `[0, 1000]`.
	/// The range of component `a` is `[0, 1000]`.
	fn destroy(n: u32, m: u32, a: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `257 + n * (76 ±0) + m * (56 ±0) + a * (107 ±0)`
		//  Estimated: `9210 + n * (2597 ±0) + m * (2662 ±0) + a * (2647 ±0)`
		// Minimum execution time: 2_372_883_000 picoseconds.
		Weight::from_parts(2_395_745_000, 0)
			.saturating_add(Weight::from_parts(0, 9210))
			// Standard Error: 24_525
			.saturating_add(Weight::from_parts(6_394_540, 0).saturating_mul(n.into()))
			// Standard Error: 24_525
			.saturating_add(Weight::from_parts(258_600, 0).saturating_mul(m.into()))
			// Standard Error: 24_525
			.saturating_add(Weight::from_parts(326_649, 0).saturating_mul(a.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(m.into())))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(a.into())))
			.saturating_add(T::DbWeight::get().writes(4))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(m.into())))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(a.into())))
			.saturating_add(Weight::from_parts(0, 2597).saturating_mul(n.into()))
			.saturating_add(Weight::from_parts(0, 2662).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 2647).saturating_mul(a.into()))
	}
	/// Storage: Uniques Asset (r:1 w:1)
	/// Proof: Uniques Asset (max_values: None, max_size: Some(122), added: 2597, mode: MaxEncodedLen)
	/// Storage: Uniques Class (r:1 w:1)
	/// Proof: Uniques Class (max_values: None, max_size: Some(178), added: 2653, mode: MaxEncodedLen)
	/// Storage: Uniques CollectionMaxSupply (r:1 w:0)
	/// Proof: Uniques CollectionMaxSupply (max_values: None, max_size: Some(24), added: 2499, mode: MaxEncodedLen)
	/// Storage: Uniques Account (r:0 w:1)
	/// Proof: Uniques Account (max_values: None, max_size: Some(88), added: 2563, mode: MaxEncodedLen)
	fn mint() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `282`
		//  Estimated: `10719`
		// Minimum execution time: 32_585_000 picoseconds.
		Weight::from_parts(33_075_000, 0)
			.saturating_add(Weight::from_parts(0, 10719))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Uniques Class (r:1 w:1)
	/// Proof: Uniques Class (max_values: None, max_size: Some(178), added: 2653, mode: MaxEncodedLen)
	/// Storage: Uniques Asset (r:1 w:1)
	/// Proof: Uniques Asset (max_values: None, max_size: Some(122), added: 2597, mode: MaxEncodedLen)
	/// Storage: Uniques Account (r:0 w:1)
	/// Proof: Uniques Account (max_values: None, max_size: Some(88), added: 2563, mode: MaxEncodedLen)
	/// Storage: Uniques ItemPriceOf (r:0 w:1)
	/// Proof: Uniques ItemPriceOf (max_values: None, max_size: Some(89), added: 2564, mode: MaxEncodedLen)
	fn burn() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `428`
		//  Estimated: `7230`
		// Minimum execution time: 33_635_000 picoseconds.
		Weight::from_parts(34_522_000, 0)
			.saturating_add(Weight::from_parts(0, 7230))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: Uniques Class (r:1 w:0)
	/// Proof: Uniques Class (max_values: None, max_size: Some(178), added: 2653, mode: MaxEncodedLen)
	/// Storage: Uniques Asset (r:1 w:1)
	/// Proof: Uniques Asset (max_values: None, max_size: Some(122), added: 2597, mode: MaxEncodedLen)
	/// Storage: Uniques Account (r:0 w:2)
	/// Proof: Uniques Account (max_values: None, max_size: Some(88), added: 2563, mode: MaxEncodedLen)
	/// Storage: Uniques ItemPriceOf (r:0 w:1)
	/// Proof: Uniques ItemPriceOf (max_values: None, max_size: Some(89), added: 2564, mode: MaxEncodedLen)
	fn transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `428`
		//  Estimated: `7230`
		// Minimum execution time: 27_011_000 picoseconds.
		Weight::from_parts(27_401_000, 0)
			.saturating_add(Weight::from_parts(0, 7230))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: Uniques Class (r:1 w:1)
	/// Proof: Uniques Class (max_values: None, max_size: Some(178), added: 2653, mode: MaxEncodedLen)
	/// Storage: Uniques Asset (r:5000 w:5000)
	/// Proof: Uniques Asset (max_values: None, max_size: Some(122), added: 2597, mode: MaxEncodedLen)
	/// The range of component `i` is `[0, 5000]`.
	fn redeposit(i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `738 + i * (76 ±0)`
		//  Estimated: `4633 + i * (2597 ±0)`
		// Minimum execution time: 15_636_000 picoseconds.
		Weight::from_parts(15_780_000, 0)
			.saturating_add(Weight::from_parts(0, 4633))
			// Standard Error: 12_666
			.saturating_add(Weight::from_parts(13_412_465, 0).saturating_mul(i.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(i.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(i.into())))
			.saturating_add(Weight::from_parts(0, 2597).saturating_mul(i.into()))
	}
	/// Storage: Uniques Asset (r:1 w:1)
	/// Proof: Uniques Asset (max_values: None, max_size: Some(122), added: 2597, mode: MaxEncodedLen)
	/// Storage: Uniques Class (r:1 w:0)
	/// Proof: Uniques Class (max_values: None, max_size: Some(178), added: 2653, mode: MaxEncodedLen)
	fn freeze() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `428`
		//  Estimated: `7230`
		// Minimum execution time: 19_152_000 picoseconds.
		Weight::from_parts(19_608_000, 0)
			.saturating_add(Weight::from_parts(0, 7230))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Uniques Asset (r:1 w:1)
	/// Proof: Uniques Asset (max_values: None, max_size: Some(122), added: 2597, mode: MaxEncodedLen)
	/// Storage: Uniques Class (r:1 w:0)
	/// Proof: Uniques Class (max_values: None, max_size: Some(178), added: 2653, mode: MaxEncodedLen)
	fn thaw() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `428`
		//  Estimated: `7230`
		// Minimum execution time: 19_463_000 picoseconds.
		Weight::from_parts(19_787_000, 0)
			.saturating_add(Weight::from_parts(0, 7230))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Uniques Class (r:1 w:1)
	/// Proof: Uniques Class (max_values: None, max_size: Some(178), added: 2653, mode: MaxEncodedLen)
	fn freeze_collection() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `282`
		//  Estimated: `3643`
		// Minimum execution time: 14_685_000 picoseconds.
		Weight::from_parts(15_044_000, 0)
			.saturating_add(Weight::from_parts(0, 3643))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Uniques Class (r:1 w:1)
	/// Proof: Uniques Class (max_values: None, max_size: Some(178), added: 2653, mode: MaxEncodedLen)
	fn thaw_collection() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `282`
		//  Estimated: `3643`
		// Minimum execution time: 14_721_000 picoseconds.
		Weight::from_parts(15_091_000, 0)
			.saturating_add(Weight::from_parts(0, 3643))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Uniques OwnershipAcceptance (r:1 w:1)
	/// Proof: Uniques OwnershipAcceptance (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	/// Storage: Uniques Class (r:1 w:1)
	/// Proof: Uniques Class (max_values: None, max_size: Some(178), added: 2653, mode: MaxEncodedLen)
	/// Storage: Uniques ClassAccount (r:0 w:2)
	/// Proof: Uniques ClassAccount (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	fn transfer_ownership() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `356`
		//  Estimated: `7160`
		// Minimum execution time: 22_435_000 picoseconds.
		Weight::from_parts(22_836_000, 0)
			.saturating_add(Weight::from_parts(0, 7160))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: Uniques Class (r:1 w:1)
	/// Proof: Uniques Class (max_values: None, max_size: Some(178), added: 2653, mode: MaxEncodedLen)
	fn set_team() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `282`
		//  Estimated: `3643`
		// Minimum execution time: 15_460_000 picoseconds.
		Weight::from_parts(15_952_000, 0)
			.saturating_add(Weight::from_parts(0, 3643))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Uniques Class (r:1 w:1)
	/// Proof: Uniques Class (max_values: None, max_size: Some(178), added: 2653, mode: MaxEncodedLen)
	/// Storage: Uniques ClassAccount (r:0 w:1)
	/// Proof: Uniques ClassAccount (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	fn force_item_status() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `282`
		//  Estimated: `3643`
		// Minimum execution time: 18_100_000 picoseconds.
		Weight::from_parts(18_369_000, 0)
			.saturating_add(Weight::from_parts(0, 3643))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Uniques Class (r:1 w:1)
	/// Proof: Uniques Class (max_values: None, max_size: Some(178), added: 2653, mode: MaxEncodedLen)
	/// Storage: Uniques InstanceMetadataOf (r:1 w:0)
	/// Proof: Uniques InstanceMetadataOf (max_values: None, max_size: Some(187), added: 2662, mode: MaxEncodedLen)
	/// Storage: Uniques Attribute (r:1 w:1)
	/// Proof: Uniques Attribute (max_values: None, max_size: Some(172), added: 2647, mode: MaxEncodedLen)
	fn set_attribute() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `559`
		//  Estimated: `10932`
		// Minimum execution time: 39_045_000 picoseconds.
		Weight::from_parts(39_588_000, 0)
			.saturating_add(Weight::from_parts(0, 10932))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Uniques Class (r:1 w:1)
	/// Proof: Uniques Class (max_values: None, max_size: Some(178), added: 2653, mode: MaxEncodedLen)
	/// Storage: Uniques InstanceMetadataOf (r:1 w:0)
	/// Proof: Uniques InstanceMetadataOf (max_values: None, max_size: Some(187), added: 2662, mode: MaxEncodedLen)
	/// Storage: Uniques Attribute (r:1 w:1)
	/// Proof: Uniques Attribute (max_values: None, max_size: Some(172), added: 2647, mode: MaxEncodedLen)
	fn clear_attribute() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `756`
		//  Estimated: `10932`
		// Minimum execution time: 37_824_000 picoseconds.
		Weight::from_parts(38_257_000, 0)
			.saturating_add(Weight::from_parts(0, 10932))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Uniques Class (r:1 w:1)
	/// Proof: Uniques Class (max_values: None, max_size: Some(178), added: 2653, mode: MaxEncodedLen)
	/// Storage: Uniques InstanceMetadataOf (r:1 w:1)
	/// Proof: Uniques InstanceMetadataOf (max_values: None, max_size: Some(187), added: 2662, mode: MaxEncodedLen)
	fn set_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `348`
		//  Estimated: `7295`
		// Minimum execution time: 30_181_000 picoseconds.
		Weight::from_parts(30_531_000, 0)
			.saturating_add(Weight::from_parts(0, 7295))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Uniques Class (r:1 w:1)
	/// Proof: Uniques Class (max_values: None, max_size: Some(178), added: 2653, mode: MaxEncodedLen)
	/// Storage: Uniques InstanceMetadataOf (r:1 w:1)
	/// Proof: Uniques InstanceMetadataOf (max_values: None, max_size: Some(187), added: 2662, mode: MaxEncodedLen)
	fn clear_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `559`
		//  Estimated: `7295`
		// Minimum execution time: 30_260_000 picoseconds.
		Weight::from_parts(30_606_000, 0)
			.saturating_add(Weight::from_parts(0, 7295))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Uniques Class (r:1 w:1)
	/// Proof: Uniques Class (max_values: None, max_size: Some(178), added: 2653, mode: MaxEncodedLen)
	/// Storage: Uniques ClassMetadataOf (r:1 w:1)
	/// Proof: Uniques ClassMetadataOf (max_values: None, max_size: Some(167), added: 2642, mode: MaxEncodedLen)
	fn set_collection_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `282`
		//  Estimated: `7275`
		// Minimum execution time: 28_698_000 picoseconds.
		Weight::from_parts(29_441_000, 0)
			.saturating_add(Weight::from_parts(0, 7275))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Uniques Class (r:1 w:0)
	/// Proof: Uniques Class (max_values: None, max_size: Some(178), added: 2653, mode: MaxEncodedLen)
	/// Storage: Uniques ClassMetadataOf (r:1 w:1)
	/// Proof: Uniques ClassMetadataOf (max_values: None, max_size: Some(167), added: 2642, mode: MaxEncodedLen)
	fn clear_collection_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `473`
		//  Estimated: `7275`
		// Minimum execution time: 27_603_000 picoseconds.
		Weight::from_parts(28_379_000, 0)
			.saturating_add(Weight::from_parts(0, 7275))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Uniques Class (r:1 w:0)
	/// Proof: Uniques Class (max_values: None, max_size: Some(178), added: 2653, mode: MaxEncodedLen)
	/// Storage: Uniques Asset (r:1 w:1)
	/// Proof: Uniques Asset (max_values: None, max_size: Some(122), added: 2597, mode: MaxEncodedLen)
	fn approve_transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `428`
		//  Estimated: `7230`
		// Minimum execution time: 21_329_000 picoseconds.
		Weight::from_parts(21_592_000, 0)
			.saturating_add(Weight::from_parts(0, 7230))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Uniques Class (r:1 w:0)
	/// Proof: Uniques Class (max_values: None, max_size: Some(178), added: 2653, mode: MaxEncodedLen)
	/// Storage: Uniques Asset (r:1 w:1)
	/// Proof: Uniques Asset (max_values: None, max_size: Some(122), added: 2597, mode: MaxEncodedLen)
	fn cancel_approval() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `461`
		//  Estimated: `7230`
		// Minimum execution time: 21_149_000 picoseconds.
		Weight::from_parts(21_340_000, 0)
			.saturating_add(Weight::from_parts(0, 7230))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Uniques OwnershipAcceptance (r:1 w:1)
	/// Proof: Uniques OwnershipAcceptance (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	fn set_accept_ownership() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42`
		//  Estimated: `3517`
		// Minimum execution time: 16_875_000 picoseconds.
		Weight::from_parts(17_221_000, 0)
			.saturating_add(Weight::from_parts(0, 3517))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Uniques CollectionMaxSupply (r:1 w:1)
	/// Proof: Uniques CollectionMaxSupply (max_values: None, max_size: Some(24), added: 2499, mode: MaxEncodedLen)
	/// Storage: Uniques Class (r:1 w:0)
	/// Proof: Uniques Class (max_values: None, max_size: Some(178), added: 2653, mode: MaxEncodedLen)
	fn set_collection_max_supply() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `282`
		//  Estimated: `7132`
		// Minimum execution time: 17_364_000 picoseconds.
		Weight::from_parts(17_723_000, 0)
			.saturating_add(Weight::from_parts(0, 7132))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Uniques Asset (r:1 w:0)
	/// Proof: Uniques Asset (max_values: None, max_size: Some(122), added: 2597, mode: MaxEncodedLen)
	/// Storage: Uniques ItemPriceOf (r:0 w:1)
	/// Proof: Uniques ItemPriceOf (max_values: None, max_size: Some(89), added: 2564, mode: MaxEncodedLen)
	fn set_price() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `259`
		//  Estimated: `3587`
		// Minimum execution time: 17_060_000 picoseconds.
		Weight::from_parts(17_411_000, 0)
			.saturating_add(Weight::from_parts(0, 3587))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Uniques Asset (r:1 w:1)
	/// Proof: Uniques Asset (max_values: None, max_size: Some(122), added: 2597, mode: MaxEncodedLen)
	/// Storage: Uniques ItemPriceOf (r:1 w:1)
	/// Proof: Uniques ItemPriceOf (max_values: None, max_size: Some(89), added: 2564, mode: MaxEncodedLen)
	/// Storage: Uniques Class (r:1 w:0)
	/// Proof: Uniques Class (max_values: None, max_size: Some(178), added: 2653, mode: MaxEncodedLen)
	/// Storage: Uniques Account (r:0 w:2)
	/// Proof: Uniques Account (max_values: None, max_size: Some(88), added: 2563, mode: MaxEncodedLen)
	fn buy_item() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `540`
		//  Estimated: `10784`
		// Minimum execution time: 37_832_000 picoseconds.
		Weight::from_parts(38_376_000, 0)
			.saturating_add(Weight::from_parts(0, 10784))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(4))
	}
}
