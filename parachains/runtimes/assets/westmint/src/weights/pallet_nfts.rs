// Copyright Parity Technologies (UK) Ltd.
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

//! Autogenerated weights for `pallet_nfts`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-03-23, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bm6`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("westmint-dev"), DB CACHE: 1024

// Executed Command:
// ./artifacts/polkadot-parachain
// benchmark
// pallet
// --chain=westmint-dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=pallet_nfts
// --extrinsic=*
// --steps=50
// --repeat=20
// --json
// --header=./file_header.txt
// --output=./parachains/runtimes/assets/westmint/src/weights/pallet_nfts.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_nfts`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_nfts::WeightInfo for WeightInfo<T> {
	/// Storage: Nfts NextCollectionId (r:1 w:1)
	/// Proof: Nfts NextCollectionId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Nfts Collection (r:1 w:1)
	/// Proof: Nfts Collection (max_values: None, max_size: Some(84), added: 2559, mode: MaxEncodedLen)
	/// Storage: Nfts CollectionRoleOf (r:0 w:1)
	/// Proof: Nfts CollectionRoleOf (max_values: None, max_size: Some(69), added: 2544, mode: MaxEncodedLen)
	/// Storage: Nfts CollectionConfigOf (r:0 w:1)
	/// Proof: Nfts CollectionConfigOf (max_values: None, max_size: Some(73), added: 2548, mode: MaxEncodedLen)
	/// Storage: Nfts CollectionAccount (r:0 w:1)
	/// Proof: Nfts CollectionAccount (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	fn create() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `145`
		//  Estimated: `5038`
		// Minimum execution time: 34_344_000 picoseconds.
		Weight::from_parts(35_251_000, 0)
			.saturating_add(Weight::from_parts(0, 5038))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: Nfts NextCollectionId (r:1 w:1)
	/// Proof: Nfts NextCollectionId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Nfts Collection (r:1 w:1)
	/// Proof: Nfts Collection (max_values: None, max_size: Some(84), added: 2559, mode: MaxEncodedLen)
	/// Storage: Nfts CollectionRoleOf (r:0 w:1)
	/// Proof: Nfts CollectionRoleOf (max_values: None, max_size: Some(69), added: 2544, mode: MaxEncodedLen)
	/// Storage: Nfts CollectionConfigOf (r:0 w:1)
	/// Proof: Nfts CollectionConfigOf (max_values: None, max_size: Some(73), added: 2548, mode: MaxEncodedLen)
	/// Storage: Nfts CollectionAccount (r:0 w:1)
	/// Proof: Nfts CollectionAccount (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	fn force_create() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42`
		//  Estimated: `5038`
		// Minimum execution time: 22_704_000 picoseconds.
		Weight::from_parts(23_146_000, 0)
			.saturating_add(Weight::from_parts(0, 5038))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: Nfts Collection (r:1 w:1)
	/// Proof: Nfts Collection (max_values: None, max_size: Some(84), added: 2559, mode: MaxEncodedLen)
	/// Storage: Nfts ItemMetadataOf (r:1 w:0)
	/// Proof: Nfts ItemMetadataOf (max_values: None, max_size: Some(347), added: 2822, mode: MaxEncodedLen)
	/// Storage: Nfts CollectionRoleOf (r:1 w:1)
	/// Proof: Nfts CollectionRoleOf (max_values: None, max_size: Some(69), added: 2544, mode: MaxEncodedLen)
	/// Storage: Nfts Attribute (r:1001 w:1000)
	/// Proof: Nfts Attribute (max_values: None, max_size: Some(479), added: 2954, mode: MaxEncodedLen)
	/// Storage: Nfts ItemConfigOf (r:1000 w:1000)
	/// Proof: Nfts ItemConfigOf (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Nfts CollectionMetadataOf (r:0 w:1)
	/// Proof: Nfts CollectionMetadataOf (max_values: None, max_size: Some(294), added: 2769, mode: MaxEncodedLen)
	/// Storage: Nfts CollectionConfigOf (r:0 w:1)
	/// Proof: Nfts CollectionConfigOf (max_values: None, max_size: Some(73), added: 2548, mode: MaxEncodedLen)
	/// Storage: Nfts CollectionAccount (r:0 w:1)
	/// Proof: Nfts CollectionAccount (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	/// The range of component `m` is `[0, 1000]`.
	/// The range of component `c` is `[0, 1000]`.
	/// The range of component `a` is `[0, 1000]`.
	fn destroy(_m: u32, _c: u32, a: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `32170 + a * (366 ±0)`
		//  Estimated: `2538829 + a * (2954 ±0)`
		// Minimum execution time: 976_206_000 picoseconds.
		Weight::from_parts(924_770_064, 0)
			.saturating_add(Weight::from_parts(0, 2538829))
			// Standard Error: 3_946
			.saturating_add(Weight::from_parts(5_708_229, 0).saturating_mul(a.into()))
			.saturating_add(T::DbWeight::get().reads(1004))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(a.into())))
			.saturating_add(T::DbWeight::get().writes(1005))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(a.into())))
			.saturating_add(Weight::from_parts(0, 2954).saturating_mul(a.into()))
	}
	/// Storage: Nfts CollectionConfigOf (r:1 w:0)
	/// Proof: Nfts CollectionConfigOf (max_values: None, max_size: Some(73), added: 2548, mode: MaxEncodedLen)
	/// Storage: Nfts Item (r:1 w:1)
	/// Proof: Nfts Item (max_values: None, max_size: Some(861), added: 3336, mode: MaxEncodedLen)
	/// Storage: Nfts Collection (r:1 w:1)
	/// Proof: Nfts Collection (max_values: None, max_size: Some(84), added: 2559, mode: MaxEncodedLen)
	/// Storage: Nfts CollectionRoleOf (r:1 w:0)
	/// Proof: Nfts CollectionRoleOf (max_values: None, max_size: Some(69), added: 2544, mode: MaxEncodedLen)
	/// Storage: Nfts ItemConfigOf (r:1 w:1)
	/// Proof: Nfts ItemConfigOf (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Nfts Account (r:0 w:1)
	/// Proof: Nfts Account (max_values: None, max_size: Some(88), added: 2563, mode: MaxEncodedLen)
	fn mint() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `421`
		//  Estimated: `18460`
		// Minimum execution time: 44_592_000 picoseconds.
		Weight::from_parts(45_181_000, 0)
			.saturating_add(Weight::from_parts(0, 18460))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: Nfts CollectionRoleOf (r:1 w:0)
	/// Proof: Nfts CollectionRoleOf (max_values: None, max_size: Some(69), added: 2544, mode: MaxEncodedLen)
	/// Storage: Nfts Item (r:1 w:1)
	/// Proof: Nfts Item (max_values: None, max_size: Some(861), added: 3336, mode: MaxEncodedLen)
	/// Storage: Nfts Collection (r:1 w:1)
	/// Proof: Nfts Collection (max_values: None, max_size: Some(84), added: 2559, mode: MaxEncodedLen)
	/// Storage: Nfts CollectionConfigOf (r:1 w:0)
	/// Proof: Nfts CollectionConfigOf (max_values: None, max_size: Some(73), added: 2548, mode: MaxEncodedLen)
	/// Storage: Nfts ItemConfigOf (r:1 w:1)
	/// Proof: Nfts ItemConfigOf (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Nfts Account (r:0 w:1)
	/// Proof: Nfts Account (max_values: None, max_size: Some(88), added: 2563, mode: MaxEncodedLen)
	fn force_mint() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `421`
		//  Estimated: `18460`
		// Minimum execution time: 43_304_000 picoseconds.
		Weight::from_parts(43_977_000, 0)
			.saturating_add(Weight::from_parts(0, 18460))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: Nfts ItemConfigOf (r:1 w:1)
	/// Proof: Nfts ItemConfigOf (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Nfts Collection (r:1 w:1)
	/// Proof: Nfts Collection (max_values: None, max_size: Some(84), added: 2559, mode: MaxEncodedLen)
	/// Storage: Nfts Item (r:1 w:1)
	/// Proof: Nfts Item (max_values: None, max_size: Some(861), added: 3336, mode: MaxEncodedLen)
	/// Storage: Nfts ItemMetadataOf (r:1 w:0)
	/// Proof: Nfts ItemMetadataOf (max_values: None, max_size: Some(347), added: 2822, mode: MaxEncodedLen)
	/// Storage: Nfts Account (r:0 w:1)
	/// Proof: Nfts Account (max_values: None, max_size: Some(88), added: 2563, mode: MaxEncodedLen)
	/// Storage: Nfts ItemPriceOf (r:0 w:1)
	/// Proof: Nfts ItemPriceOf (max_values: None, max_size: Some(89), added: 2564, mode: MaxEncodedLen)
	/// Storage: Nfts ItemAttributesApprovalsOf (r:0 w:1)
	/// Proof: Nfts ItemAttributesApprovalsOf (max_values: None, max_size: Some(1001), added: 3476, mode: MaxEncodedLen)
	/// Storage: Nfts PendingSwapOf (r:0 w:1)
	/// Proof: Nfts PendingSwapOf (max_values: None, max_size: Some(71), added: 2546, mode: MaxEncodedLen)
	fn burn() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `530`
		//  Estimated: `15200`
		// Minimum execution time: 45_744_000 picoseconds.
		Weight::from_parts(46_056_000, 0)
			.saturating_add(Weight::from_parts(0, 15200))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(7))
	}
	/// Storage: Nfts Collection (r:1 w:0)
	/// Proof: Nfts Collection (max_values: None, max_size: Some(84), added: 2559, mode: MaxEncodedLen)
	/// Storage: Nfts CollectionConfigOf (r:1 w:0)
	/// Proof: Nfts CollectionConfigOf (max_values: None, max_size: Some(73), added: 2548, mode: MaxEncodedLen)
	/// Storage: Nfts ItemConfigOf (r:1 w:0)
	/// Proof: Nfts ItemConfigOf (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Nfts Item (r:1 w:1)
	/// Proof: Nfts Item (max_values: None, max_size: Some(861), added: 3336, mode: MaxEncodedLen)
	/// Storage: Nfts Account (r:0 w:2)
	/// Proof: Nfts Account (max_values: None, max_size: Some(88), added: 2563, mode: MaxEncodedLen)
	/// Storage: Nfts ItemPriceOf (r:0 w:1)
	/// Proof: Nfts ItemPriceOf (max_values: None, max_size: Some(89), added: 2564, mode: MaxEncodedLen)
	/// Storage: Nfts PendingSwapOf (r:0 w:1)
	/// Proof: Nfts PendingSwapOf (max_values: None, max_size: Some(71), added: 2546, mode: MaxEncodedLen)
	fn transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `559`
		//  Estimated: `14926`
		// Minimum execution time: 35_663_000 picoseconds.
		Weight::from_parts(36_865_000, 0)
			.saturating_add(Weight::from_parts(0, 14926))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: Nfts Collection (r:1 w:0)
	/// Proof: Nfts Collection (max_values: None, max_size: Some(84), added: 2559, mode: MaxEncodedLen)
	/// Storage: Nfts CollectionConfigOf (r:1 w:0)
	/// Proof: Nfts CollectionConfigOf (max_values: None, max_size: Some(73), added: 2548, mode: MaxEncodedLen)
	/// Storage: Nfts Item (r:5000 w:5000)
	/// Proof: Nfts Item (max_values: None, max_size: Some(861), added: 3336, mode: MaxEncodedLen)
	/// The range of component `i` is `[0, 5000]`.
	fn redeposit(i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `729 + i * (108 ±0)`
		//  Estimated: `8077 + i * (3336 ±0)`
		// Minimum execution time: 16_987_000 picoseconds.
		Weight::from_parts(17_194_000, 0)
			.saturating_add(Weight::from_parts(0, 8077))
			// Standard Error: 13_044
			.saturating_add(Weight::from_parts(13_324_147, 0).saturating_mul(i.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(i.into())))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(i.into())))
			.saturating_add(Weight::from_parts(0, 3336).saturating_mul(i.into()))
	}
	/// Storage: Nfts CollectionRoleOf (r:1 w:0)
	/// Proof: Nfts CollectionRoleOf (max_values: None, max_size: Some(69), added: 2544, mode: MaxEncodedLen)
	/// Storage: Nfts ItemConfigOf (r:1 w:1)
	/// Proof: Nfts ItemConfigOf (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	fn lock_item_transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `401`
		//  Estimated: `7047`
		// Minimum execution time: 20_345_000 picoseconds.
		Weight::from_parts(20_739_000, 0)
			.saturating_add(Weight::from_parts(0, 7047))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Nfts CollectionRoleOf (r:1 w:0)
	/// Proof: Nfts CollectionRoleOf (max_values: None, max_size: Some(69), added: 2544, mode: MaxEncodedLen)
	/// Storage: Nfts ItemConfigOf (r:1 w:1)
	/// Proof: Nfts ItemConfigOf (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	fn unlock_item_transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `401`
		//  Estimated: `7047`
		// Minimum execution time: 20_167_000 picoseconds.
		Weight::from_parts(20_580_000, 0)
			.saturating_add(Weight::from_parts(0, 7047))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Nfts Collection (r:1 w:0)
	/// Proof: Nfts Collection (max_values: None, max_size: Some(84), added: 2559, mode: MaxEncodedLen)
	/// Storage: Nfts CollectionConfigOf (r:1 w:1)
	/// Proof: Nfts CollectionConfigOf (max_values: None, max_size: Some(73), added: 2548, mode: MaxEncodedLen)
	fn lock_collection() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `306`
		//  Estimated: `7087`
		// Minimum execution time: 17_831_000 picoseconds.
		Weight::from_parts(18_174_000, 0)
			.saturating_add(Weight::from_parts(0, 7087))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Nfts OwnershipAcceptance (r:1 w:1)
	/// Proof: Nfts OwnershipAcceptance (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	/// Storage: Nfts Collection (r:1 w:1)
	/// Proof: Nfts Collection (max_values: None, max_size: Some(84), added: 2559, mode: MaxEncodedLen)
	/// Storage: Nfts CollectionAccount (r:0 w:2)
	/// Proof: Nfts CollectionAccount (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	fn transfer_ownership() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `354`
		//  Estimated: `7066`
		// Minimum execution time: 23_763_000 picoseconds.
		Weight::from_parts(24_226_000, 0)
			.saturating_add(Weight::from_parts(0, 7066))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: Nfts Collection (r:1 w:1)
	/// Proof: Nfts Collection (max_values: None, max_size: Some(84), added: 2559, mode: MaxEncodedLen)
	/// Storage: Nfts CollectionRoleOf (r:2 w:4)
	/// Proof: Nfts CollectionRoleOf (max_values: None, max_size: Some(69), added: 2544, mode: MaxEncodedLen)
	fn set_team() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `335`
		//  Estimated: `9627`
		// Minimum execution time: 40_034_000 picoseconds.
		Weight::from_parts(40_402_000, 0)
			.saturating_add(Weight::from_parts(0, 9627))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: Nfts Collection (r:1 w:1)
	/// Proof: Nfts Collection (max_values: None, max_size: Some(84), added: 2559, mode: MaxEncodedLen)
	/// Storage: Nfts CollectionAccount (r:0 w:2)
	/// Proof: Nfts CollectionAccount (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	fn force_collection_owner() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `277`
		//  Estimated: `3549`
		// Minimum execution time: 18_648_000 picoseconds.
		Weight::from_parts(18_968_000, 0)
			.saturating_add(Weight::from_parts(0, 3549))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Nfts Collection (r:1 w:0)
	/// Proof: Nfts Collection (max_values: None, max_size: Some(84), added: 2559, mode: MaxEncodedLen)
	/// Storage: Nfts CollectionConfigOf (r:0 w:1)
	/// Proof: Nfts CollectionConfigOf (max_values: None, max_size: Some(73), added: 2548, mode: MaxEncodedLen)
	fn force_collection_config() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `242`
		//  Estimated: `3549`
		// Minimum execution time: 15_282_000 picoseconds.
		Weight::from_parts(15_923_000, 0)
			.saturating_add(Weight::from_parts(0, 3549))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Nfts CollectionRoleOf (r:1 w:0)
	/// Proof: Nfts CollectionRoleOf (max_values: None, max_size: Some(69), added: 2544, mode: MaxEncodedLen)
	/// Storage: Nfts ItemConfigOf (r:1 w:1)
	/// Proof: Nfts ItemConfigOf (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	fn lock_item_properties() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `401`
		//  Estimated: `7047`
		// Minimum execution time: 20_060_000 picoseconds.
		Weight::from_parts(20_326_000, 0)
			.saturating_add(Weight::from_parts(0, 7047))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Nfts Collection (r:1 w:1)
	/// Proof: Nfts Collection (max_values: None, max_size: Some(84), added: 2559, mode: MaxEncodedLen)
	/// Storage: Nfts CollectionRoleOf (r:1 w:0)
	/// Proof: Nfts CollectionRoleOf (max_values: None, max_size: Some(69), added: 2544, mode: MaxEncodedLen)
	/// Storage: Nfts CollectionConfigOf (r:1 w:0)
	/// Proof: Nfts CollectionConfigOf (max_values: None, max_size: Some(73), added: 2548, mode: MaxEncodedLen)
	/// Storage: Nfts ItemConfigOf (r:1 w:0)
	/// Proof: Nfts ItemConfigOf (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Nfts Attribute (r:1 w:1)
	/// Proof: Nfts Attribute (max_values: None, max_size: Some(479), added: 2954, mode: MaxEncodedLen)
	fn set_attribute() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `505`
		//  Estimated: `18078`
		// Minimum execution time: 48_324_000 picoseconds.
		Weight::from_parts(48_745_000, 0)
			.saturating_add(Weight::from_parts(0, 18078))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Nfts Collection (r:1 w:1)
	/// Proof: Nfts Collection (max_values: None, max_size: Some(84), added: 2559, mode: MaxEncodedLen)
	/// Storage: Nfts Attribute (r:1 w:1)
	/// Proof: Nfts Attribute (max_values: None, max_size: Some(479), added: 2954, mode: MaxEncodedLen)
	fn force_set_attribute() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `310`
		//  Estimated: `7493`
		// Minimum execution time: 27_935_000 picoseconds.
		Weight::from_parts(28_241_000, 0)
			.saturating_add(Weight::from_parts(0, 7493))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Nfts Attribute (r:1 w:1)
	/// Proof: Nfts Attribute (max_values: None, max_size: Some(479), added: 2954, mode: MaxEncodedLen)
	/// Storage: Nfts CollectionRoleOf (r:1 w:0)
	/// Proof: Nfts CollectionRoleOf (max_values: None, max_size: Some(69), added: 2544, mode: MaxEncodedLen)
	/// Storage: Nfts ItemConfigOf (r:1 w:0)
	/// Proof: Nfts ItemConfigOf (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Nfts Collection (r:1 w:1)
	/// Proof: Nfts Collection (max_values: None, max_size: Some(84), added: 2559, mode: MaxEncodedLen)
	fn clear_attribute() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `949`
		//  Estimated: `14540`
		// Minimum execution time: 44_972_000 picoseconds.
		Weight::from_parts(45_618_000, 0)
			.saturating_add(Weight::from_parts(0, 14540))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Nfts Item (r:1 w:0)
	/// Proof: Nfts Item (max_values: None, max_size: Some(861), added: 3336, mode: MaxEncodedLen)
	/// Storage: Nfts ItemAttributesApprovalsOf (r:1 w:1)
	/// Proof: Nfts ItemAttributesApprovalsOf (max_values: None, max_size: Some(1001), added: 3476, mode: MaxEncodedLen)
	fn approve_item_attributes() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `347`
		//  Estimated: `8792`
		// Minimum execution time: 19_246_000 picoseconds.
		Weight::from_parts(19_715_000, 0)
			.saturating_add(Weight::from_parts(0, 8792))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Nfts Item (r:1 w:0)
	/// Proof: Nfts Item (max_values: None, max_size: Some(861), added: 3336, mode: MaxEncodedLen)
	/// Storage: Nfts ItemAttributesApprovalsOf (r:1 w:1)
	/// Proof: Nfts ItemAttributesApprovalsOf (max_values: None, max_size: Some(1001), added: 3476, mode: MaxEncodedLen)
	/// Storage: Nfts Attribute (r:1001 w:1000)
	/// Proof: Nfts Attribute (max_values: None, max_size: Some(479), added: 2954, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `n` is `[0, 1000]`.
	fn cancel_item_attributes_approval(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `726 + n * (398 ±0)`
		//  Estimated: `16329 + n * (2954 ±0)`
		// Minimum execution time: 28_372_000 picoseconds.
		Weight::from_parts(28_671_000, 0)
			.saturating_add(Weight::from_parts(0, 16329))
			// Standard Error: 3_479
			.saturating_add(Weight::from_parts(5_527_336, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(n.into())))
			.saturating_add(Weight::from_parts(0, 2954).saturating_mul(n.into()))
	}
	/// Storage: Nfts CollectionRoleOf (r:1 w:0)
	/// Proof: Nfts CollectionRoleOf (max_values: None, max_size: Some(69), added: 2544, mode: MaxEncodedLen)
	/// Storage: Nfts Collection (r:1 w:1)
	/// Proof: Nfts Collection (max_values: None, max_size: Some(84), added: 2559, mode: MaxEncodedLen)
	/// Storage: Nfts ItemConfigOf (r:1 w:0)
	/// Proof: Nfts ItemConfigOf (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Nfts CollectionConfigOf (r:1 w:0)
	/// Proof: Nfts CollectionConfigOf (max_values: None, max_size: Some(73), added: 2548, mode: MaxEncodedLen)
	/// Storage: Nfts ItemMetadataOf (r:1 w:1)
	/// Proof: Nfts ItemMetadataOf (max_values: None, max_size: Some(347), added: 2822, mode: MaxEncodedLen)
	fn set_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `505`
		//  Estimated: `17946`
		// Minimum execution time: 39_852_000 picoseconds.
		Weight::from_parts(40_280_000, 0)
			.saturating_add(Weight::from_parts(0, 17946))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Nfts CollectionRoleOf (r:1 w:0)
	/// Proof: Nfts CollectionRoleOf (max_values: None, max_size: Some(69), added: 2544, mode: MaxEncodedLen)
	/// Storage: Nfts ItemMetadataOf (r:1 w:1)
	/// Proof: Nfts ItemMetadataOf (max_values: None, max_size: Some(347), added: 2822, mode: MaxEncodedLen)
	/// Storage: Nfts Collection (r:1 w:1)
	/// Proof: Nfts Collection (max_values: None, max_size: Some(84), added: 2559, mode: MaxEncodedLen)
	/// Storage: Nfts ItemConfigOf (r:1 w:0)
	/// Proof: Nfts ItemConfigOf (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	fn clear_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `815`
		//  Estimated: `14408`
		// Minimum execution time: 36_829_000 picoseconds.
		Weight::from_parts(37_513_000, 0)
			.saturating_add(Weight::from_parts(0, 14408))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Nfts CollectionRoleOf (r:1 w:0)
	/// Proof: Nfts CollectionRoleOf (max_values: None, max_size: Some(69), added: 2544, mode: MaxEncodedLen)
	/// Storage: Nfts CollectionConfigOf (r:1 w:0)
	/// Proof: Nfts CollectionConfigOf (max_values: None, max_size: Some(73), added: 2548, mode: MaxEncodedLen)
	/// Storage: Nfts Collection (r:1 w:1)
	/// Proof: Nfts Collection (max_values: None, max_size: Some(84), added: 2559, mode: MaxEncodedLen)
	/// Storage: Nfts CollectionMetadataOf (r:1 w:1)
	/// Proof: Nfts CollectionMetadataOf (max_values: None, max_size: Some(294), added: 2769, mode: MaxEncodedLen)
	fn set_collection_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `364`
		//  Estimated: `14380`
		// Minimum execution time: 35_398_000 picoseconds.
		Weight::from_parts(35_809_000, 0)
			.saturating_add(Weight::from_parts(0, 14380))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Nfts CollectionRoleOf (r:1 w:0)
	/// Proof: Nfts CollectionRoleOf (max_values: None, max_size: Some(69), added: 2544, mode: MaxEncodedLen)
	/// Storage: Nfts Collection (r:1 w:0)
	/// Proof: Nfts Collection (max_values: None, max_size: Some(84), added: 2559, mode: MaxEncodedLen)
	/// Storage: Nfts CollectionConfigOf (r:1 w:0)
	/// Proof: Nfts CollectionConfigOf (max_values: None, max_size: Some(73), added: 2548, mode: MaxEncodedLen)
	/// Storage: Nfts CollectionMetadataOf (r:1 w:1)
	/// Proof: Nfts CollectionMetadataOf (max_values: None, max_size: Some(294), added: 2769, mode: MaxEncodedLen)
	fn clear_collection_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `682`
		//  Estimated: `14380`
		// Minimum execution time: 33_699_000 picoseconds.
		Weight::from_parts(34_170_000, 0)
			.saturating_add(Weight::from_parts(0, 14380))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Nfts Item (r:1 w:1)
	/// Proof: Nfts Item (max_values: None, max_size: Some(861), added: 3336, mode: MaxEncodedLen)
	/// Storage: Nfts CollectionConfigOf (r:1 w:0)
	/// Proof: Nfts CollectionConfigOf (max_values: None, max_size: Some(73), added: 2548, mode: MaxEncodedLen)
	fn approve_transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `376`
		//  Estimated: `7864`
		// Minimum execution time: 21_789_000 picoseconds.
		Weight::from_parts(22_454_000, 0)
			.saturating_add(Weight::from_parts(0, 7864))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Nfts Item (r:1 w:1)
	/// Proof: Nfts Item (max_values: None, max_size: Some(861), added: 3336, mode: MaxEncodedLen)
	fn cancel_approval() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `384`
		//  Estimated: `4326`
		// Minimum execution time: 19_532_000 picoseconds.
		Weight::from_parts(19_761_000, 0)
			.saturating_add(Weight::from_parts(0, 4326))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Nfts Item (r:1 w:1)
	/// Proof: Nfts Item (max_values: None, max_size: Some(861), added: 3336, mode: MaxEncodedLen)
	fn clear_all_transfer_approvals() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `384`
		//  Estimated: `4326`
		// Minimum execution time: 18_620_000 picoseconds.
		Weight::from_parts(19_014_000, 0)
			.saturating_add(Weight::from_parts(0, 4326))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Nfts OwnershipAcceptance (r:1 w:1)
	/// Proof: Nfts OwnershipAcceptance (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	fn set_accept_ownership() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42`
		//  Estimated: `3517`
		// Minimum execution time: 16_491_000 picoseconds.
		Weight::from_parts(16_888_000, 0)
			.saturating_add(Weight::from_parts(0, 3517))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Nfts CollectionConfigOf (r:1 w:1)
	/// Proof: Nfts CollectionConfigOf (max_values: None, max_size: Some(73), added: 2548, mode: MaxEncodedLen)
	/// Storage: Nfts Collection (r:1 w:0)
	/// Proof: Nfts Collection (max_values: None, max_size: Some(84), added: 2559, mode: MaxEncodedLen)
	fn set_collection_max_supply() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `306`
		//  Estimated: `7087`
		// Minimum execution time: 19_929_000 picoseconds.
		Weight::from_parts(20_170_000, 0)
			.saturating_add(Weight::from_parts(0, 7087))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Nfts CollectionRoleOf (r:1 w:0)
	/// Proof: Nfts CollectionRoleOf (max_values: None, max_size: Some(69), added: 2544, mode: MaxEncodedLen)
	/// Storage: Nfts CollectionConfigOf (r:1 w:1)
	/// Proof: Nfts CollectionConfigOf (max_values: None, max_size: Some(73), added: 2548, mode: MaxEncodedLen)
	fn update_mint_settings() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `289`
		//  Estimated: `7072`
		// Minimum execution time: 19_500_000 picoseconds.
		Weight::from_parts(19_839_000, 0)
			.saturating_add(Weight::from_parts(0, 7072))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Nfts Item (r:1 w:0)
	/// Proof: Nfts Item (max_values: None, max_size: Some(861), added: 3336, mode: MaxEncodedLen)
	/// Storage: Nfts CollectionConfigOf (r:1 w:0)
	/// Proof: Nfts CollectionConfigOf (max_values: None, max_size: Some(73), added: 2548, mode: MaxEncodedLen)
	/// Storage: Nfts ItemConfigOf (r:1 w:0)
	/// Proof: Nfts ItemConfigOf (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Nfts ItemPriceOf (r:0 w:1)
	/// Proof: Nfts ItemPriceOf (max_values: None, max_size: Some(89), added: 2564, mode: MaxEncodedLen)
	fn set_price() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `484`
		//  Estimated: `11377`
		// Minimum execution time: 24_542_000 picoseconds.
		Weight::from_parts(24_916_000, 0)
			.saturating_add(Weight::from_parts(0, 11377))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Nfts Item (r:1 w:1)
	/// Proof: Nfts Item (max_values: None, max_size: Some(861), added: 3336, mode: MaxEncodedLen)
	/// Storage: Nfts ItemPriceOf (r:1 w:1)
	/// Proof: Nfts ItemPriceOf (max_values: None, max_size: Some(89), added: 2564, mode: MaxEncodedLen)
	/// Storage: Nfts Collection (r:1 w:0)
	/// Proof: Nfts Collection (max_values: None, max_size: Some(84), added: 2559, mode: MaxEncodedLen)
	/// Storage: Nfts CollectionConfigOf (r:1 w:0)
	/// Proof: Nfts CollectionConfigOf (max_values: None, max_size: Some(73), added: 2548, mode: MaxEncodedLen)
	/// Storage: Nfts ItemConfigOf (r:1 w:0)
	/// Proof: Nfts ItemConfigOf (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Nfts Account (r:0 w:2)
	/// Proof: Nfts Account (max_values: None, max_size: Some(88), added: 2563, mode: MaxEncodedLen)
	/// Storage: Nfts PendingSwapOf (r:0 w:1)
	/// Proof: Nfts PendingSwapOf (max_values: None, max_size: Some(71), added: 2546, mode: MaxEncodedLen)
	fn buy_item() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `671`
		//  Estimated: `18480`
		// Minimum execution time: 44_311_000 picoseconds.
		Weight::from_parts(45_789_000, 0)
			.saturating_add(Weight::from_parts(0, 18480))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// The range of component `n` is `[0, 10]`.
	fn pay_tips(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_523_000 picoseconds.
		Weight::from_parts(4_349_031, 0)
			.saturating_add(Weight::from_parts(0, 0))
			// Standard Error: 10_427
			.saturating_add(Weight::from_parts(3_718_129, 0).saturating_mul(n.into()))
	}
	/// Storage: Nfts Item (r:2 w:0)
	/// Proof: Nfts Item (max_values: None, max_size: Some(861), added: 3336, mode: MaxEncodedLen)
	/// Storage: Nfts PendingSwapOf (r:0 w:1)
	/// Proof: Nfts PendingSwapOf (max_values: None, max_size: Some(71), added: 2546, mode: MaxEncodedLen)
	fn create_swap() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `460`
		//  Estimated: `7662`
		// Minimum execution time: 23_007_000 picoseconds.
		Weight::from_parts(23_305_000, 0)
			.saturating_add(Weight::from_parts(0, 7662))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Nfts PendingSwapOf (r:1 w:1)
	/// Proof: Nfts PendingSwapOf (max_values: None, max_size: Some(71), added: 2546, mode: MaxEncodedLen)
	/// Storage: Nfts Item (r:1 w:0)
	/// Proof: Nfts Item (max_values: None, max_size: Some(861), added: 3336, mode: MaxEncodedLen)
	fn cancel_swap() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `479`
		//  Estimated: `7862`
		// Minimum execution time: 21_173_000 picoseconds.
		Weight::from_parts(21_451_000, 0)
			.saturating_add(Weight::from_parts(0, 7862))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Nfts Item (r:2 w:2)
	/// Proof: Nfts Item (max_values: None, max_size: Some(861), added: 3336, mode: MaxEncodedLen)
	/// Storage: Nfts PendingSwapOf (r:1 w:2)
	/// Proof: Nfts PendingSwapOf (max_values: None, max_size: Some(71), added: 2546, mode: MaxEncodedLen)
	/// Storage: Nfts Collection (r:1 w:0)
	/// Proof: Nfts Collection (max_values: None, max_size: Some(84), added: 2559, mode: MaxEncodedLen)
	/// Storage: Nfts CollectionConfigOf (r:1 w:0)
	/// Proof: Nfts CollectionConfigOf (max_values: None, max_size: Some(73), added: 2548, mode: MaxEncodedLen)
	/// Storage: Nfts ItemConfigOf (r:2 w:0)
	/// Proof: Nfts ItemConfigOf (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: Nfts Account (r:0 w:4)
	/// Proof: Nfts Account (max_values: None, max_size: Some(88), added: 2563, mode: MaxEncodedLen)
	/// Storage: Nfts ItemPriceOf (r:0 w:2)
	/// Proof: Nfts ItemPriceOf (max_values: None, max_size: Some(89), added: 2564, mode: MaxEncodedLen)
	fn claim_swap() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `800`
		//  Estimated: `24321`
		// Minimum execution time: 72_213_000 picoseconds.
		Weight::from_parts(73_029_000, 0)
			.saturating_add(Weight::from_parts(0, 24321))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(10))
	}
	/// Storage: Nfts CollectionRoleOf (r:2 w:0)
	/// Proof: Nfts CollectionRoleOf (max_values: None, max_size: Some(69), added: 2544, mode: MaxEncodedLen)
	/// Storage: Nfts CollectionConfigOf (r:1 w:0)
	/// Proof: Nfts CollectionConfigOf (max_values: None, max_size: Some(73), added: 2548, mode: MaxEncodedLen)
	/// Storage: Nfts Item (r:1 w:1)
	/// Proof: Nfts Item (max_values: None, max_size: Some(861), added: 3336, mode: MaxEncodedLen)
	/// Storage: Nfts Collection (r:1 w:1)
	/// Proof: Nfts Collection (max_values: None, max_size: Some(84), added: 2559, mode: MaxEncodedLen)
	/// Storage: Nfts ItemConfigOf (r:1 w:1)
	/// Proof: Nfts ItemConfigOf (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Nfts Attribute (r:10 w:10)
	/// Proof: Nfts Attribute (max_values: None, max_size: Some(479), added: 2954, mode: MaxEncodedLen)
	/// Storage: Nfts ItemMetadataOf (r:1 w:1)
	/// Proof: Nfts ItemMetadataOf (max_values: None, max_size: Some(347), added: 2822, mode: MaxEncodedLen)
	/// Storage: Nfts Account (r:0 w:1)
	/// Proof: Nfts Account (max_values: None, max_size: Some(88), added: 2563, mode: MaxEncodedLen)
	/// The range of component `n` is `[0, 10]`.
	fn mint_pre_signed(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `524`
		//  Estimated: `29399 + n * (2954 ±0)`
		// Minimum execution time: 125_518_000 picoseconds.
		Weight::from_parts(129_781_908, 0)
			.saturating_add(Weight::from_parts(0, 29399))
			// Standard Error: 21_840
			.saturating_add(Weight::from_parts(26_756_136, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes(6))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(n.into())))
			.saturating_add(Weight::from_parts(0, 2954).saturating_mul(n.into()))
	}
	/// Storage: Nfts Item (r:1 w:0)
	/// Proof: Nfts Item (max_values: None, max_size: Some(861), added: 3336, mode: MaxEncodedLen)
	/// Storage: Nfts ItemAttributesApprovalsOf (r:1 w:1)
	/// Proof: Nfts ItemAttributesApprovalsOf (max_values: None, max_size: Some(1001), added: 3476, mode: MaxEncodedLen)
	/// Storage: Nfts CollectionConfigOf (r:1 w:0)
	/// Proof: Nfts CollectionConfigOf (max_values: None, max_size: Some(73), added: 2548, mode: MaxEncodedLen)
	/// Storage: Nfts Collection (r:1 w:1)
	/// Proof: Nfts Collection (max_values: None, max_size: Some(84), added: 2559, mode: MaxEncodedLen)
	/// Storage: Nfts Attribute (r:10 w:10)
	/// Proof: Nfts Attribute (max_values: None, max_size: Some(479), added: 2954, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `n` is `[0, 10]`.
	fn set_attributes_pre_signed(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `554`
		//  Estimated: `20462 + n * (2954 ±0)`
		// Minimum execution time: 76_133_000 picoseconds.
		Weight::from_parts(85_559_988, 0)
			.saturating_add(Weight::from_parts(0, 20462))
			// Standard Error: 49_851
			.saturating_add(Weight::from_parts(26_551_215, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(n.into())))
			.saturating_add(Weight::from_parts(0, 2954).saturating_mul(n.into()))
	}
}
