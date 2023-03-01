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

//! Autogenerated weights for `pallet_collective`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-03-01, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bm3`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("collectives-polkadot-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot-parachain
// benchmark
// pallet
// --chain=collectives-polkadot-dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=pallet_collective
// --no-storage-info
// --no-median-slopes
// --no-min-squares
// --extrinsic=*
// --steps=50
// --repeat=20
// --json
// --header=./file_header.txt
// --output=./parachains/runtimes/collectives/collectives-polkadot/src/weights/pallet_collective.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_collective`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_collective::WeightInfo for WeightInfo<T> {
	/// Storage: AllianceMotion Members (r:1 w:1)
	/// Proof Skipped: AllianceMotion Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AllianceMotion Proposals (r:1 w:0)
	/// Proof Skipped: AllianceMotion Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AllianceMotion Voting (r:100 w:100)
	/// Proof Skipped: AllianceMotion Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: AllianceMotion Prime (r:0 w:1)
	/// Proof Skipped: AllianceMotion Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `m` is `[0, 100]`.
	/// The range of component `n` is `[0, 100]`.
	/// The range of component `p` is `[0, 100]`.
	fn set_members(m: u32, _n: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + m * (3233 ±0) + p * (3223 ±0)`
		//  Estimated: `15906 + m * (7809 ±23) + p * (10238 ±23)`
		// Minimum execution time: 15_062 nanoseconds.
		Weight::from_ref_time(15_173_000)
			.saturating_add(Weight::from_proof_size(15906))
			// Standard Error: 66_300
			.saturating_add(Weight::from_ref_time(5_251_530).saturating_mul(m.into()))
			// Standard Error: 66_300
			.saturating_add(Weight::from_ref_time(7_810_612).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(p.into())))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(p.into())))
			.saturating_add(Weight::from_proof_size(7809).saturating_mul(m.into()))
			.saturating_add(Weight::from_proof_size(10238).saturating_mul(p.into()))
	}
	/// Storage: AllianceMotion Members (r:1 w:0)
	/// Proof Skipped: AllianceMotion Members (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 100]`.
	fn execute(b: u32, m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `64 + m * (32 ±0)`
		//  Estimated: `560 + m * (32 ±0)`
		// Minimum execution time: 13_572 nanoseconds.
		Weight::from_ref_time(13_608_404)
			.saturating_add(Weight::from_proof_size(560))
			// Standard Error: 283
			.saturating_add(Weight::from_ref_time(1_391).saturating_mul(b.into()))
			// Standard Error: 2_921
			.saturating_add(Weight::from_ref_time(17_235).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(Weight::from_proof_size(32).saturating_mul(m.into()))
	}
	/// Storage: AllianceMotion Members (r:1 w:0)
	/// Proof Skipped: AllianceMotion Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AllianceMotion ProposalOf (r:1 w:0)
	/// Proof Skipped: AllianceMotion ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 100]`.
	fn propose_execute(b: u32, m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `64 + m * (32 ±0)`
		//  Estimated: `3100 + m * (64 ±0)`
		// Minimum execution time: 15_826 nanoseconds.
		Weight::from_ref_time(14_121_816)
			.saturating_add(Weight::from_proof_size(3100))
			// Standard Error: 293
			.saturating_add(Weight::from_ref_time(3_089).saturating_mul(b.into()))
			// Standard Error: 3_024
			.saturating_add(Weight::from_ref_time(29_681).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(Weight::from_proof_size(64).saturating_mul(m.into()))
	}
	/// Storage: AllianceMotion Members (r:1 w:0)
	/// Proof Skipped: AllianceMotion Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AllianceMotion ProposalOf (r:1 w:1)
	/// Proof Skipped: AllianceMotion ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// Storage: AllianceMotion Proposals (r:1 w:1)
	/// Proof Skipped: AllianceMotion Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AllianceMotion ProposalCount (r:1 w:1)
	/// Proof Skipped: AllianceMotion ProposalCount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AllianceMotion Voting (r:0 w:1)
	/// Proof Skipped: AllianceMotion Voting (max_values: None, max_size: None, mode: Measured)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[2, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn propose_proposed(b: u32, m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `386 + m * (32 ±0) + p * (36 ±0)`
		//  Estimated: `5505 + m * (165 ±0) + p * (180 ±0)`
		// Minimum execution time: 21_647 nanoseconds.
		Weight::from_ref_time(20_758_536)
			.saturating_add(Weight::from_proof_size(5505))
			// Standard Error: 198
			.saturating_add(Weight::from_ref_time(3_765).saturating_mul(b.into()))
			// Standard Error: 2_069
			.saturating_add(Weight::from_ref_time(25_994).saturating_mul(m.into()))
			// Standard Error: 2_043
			.saturating_add(Weight::from_ref_time(103_451).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
			.saturating_add(Weight::from_proof_size(165).saturating_mul(m.into()))
			.saturating_add(Weight::from_proof_size(180).saturating_mul(p.into()))
	}
	/// Storage: AllianceMotion Members (r:1 w:0)
	/// Proof Skipped: AllianceMotion Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AllianceMotion Voting (r:1 w:1)
	/// Proof Skipped: AllianceMotion Voting (max_values: None, max_size: None, mode: Measured)
	/// The range of component `m` is `[5, 100]`.
	fn vote(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `836 + m * (64 ±0)`
		//  Estimated: `4640 + m * (128 ±0)`
		// Minimum execution time: 20_745 nanoseconds.
		Weight::from_ref_time(21_393_512)
			.saturating_add(Weight::from_proof_size(4640))
			// Standard Error: 424
			.saturating_add(Weight::from_ref_time(39_017).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_proof_size(128).saturating_mul(m.into()))
	}
	/// Storage: AllianceMotion Voting (r:1 w:1)
	/// Proof Skipped: AllianceMotion Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: AllianceMotion Members (r:1 w:0)
	/// Proof Skipped: AllianceMotion Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AllianceMotion Proposals (r:1 w:1)
	/// Proof Skipped: AllianceMotion Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AllianceMotion ProposalOf (r:0 w:1)
	/// Proof Skipped: AllianceMotion ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_early_disapproved(m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `456 + m * (64 ±0) + p * (36 ±0)`
		//  Estimated: `5213 + m * (260 ±0) + p * (144 ±0)`
		// Minimum execution time: 24_715 nanoseconds.
		Weight::from_ref_time(25_639_936)
			.saturating_add(Weight::from_proof_size(5213))
			// Standard Error: 555
			.saturating_add(Weight::from_ref_time(20_523).saturating_mul(m.into()))
			// Standard Error: 541
			.saturating_add(Weight::from_ref_time(87_097).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_proof_size(260).saturating_mul(m.into()))
			.saturating_add(Weight::from_proof_size(144).saturating_mul(p.into()))
	}
	/// Storage: AllianceMotion Voting (r:1 w:1)
	/// Proof Skipped: AllianceMotion Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: AllianceMotion Members (r:1 w:0)
	/// Proof Skipped: AllianceMotion Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AllianceMotion ProposalOf (r:1 w:1)
	/// Proof Skipped: AllianceMotion ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// Storage: AllianceMotion Proposals (r:1 w:1)
	/// Proof Skipped: AllianceMotion Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_early_approved(b: u32, m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `792 + b * (1 ±0) + m * (64 ±0) + p * (40 ±0)`
		//  Estimated: `8484 + b * (4 ±0) + m * (264 ±0) + p * (160 ±0)`
		// Minimum execution time: 34_311 nanoseconds.
		Weight::from_ref_time(35_947_869)
			.saturating_add(Weight::from_proof_size(8484))
			// Standard Error: 113
			.saturating_add(Weight::from_ref_time(1_397).saturating_mul(b.into()))
			// Standard Error: 1_199
			.saturating_add(Weight::from_ref_time(14_582).saturating_mul(m.into()))
			// Standard Error: 1_169
			.saturating_add(Weight::from_ref_time(100_613).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_proof_size(4).saturating_mul(b.into()))
			.saturating_add(Weight::from_proof_size(264).saturating_mul(m.into()))
			.saturating_add(Weight::from_proof_size(160).saturating_mul(p.into()))
	}
	/// Storage: AllianceMotion Voting (r:1 w:1)
	/// Proof Skipped: AllianceMotion Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: AllianceMotion Members (r:1 w:0)
	/// Proof Skipped: AllianceMotion Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AllianceMotion Prime (r:1 w:0)
	/// Proof Skipped: AllianceMotion Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AllianceMotion Proposals (r:1 w:1)
	/// Proof Skipped: AllianceMotion Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AllianceMotion ProposalOf (r:0 w:1)
	/// Proof Skipped: AllianceMotion ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_disapproved(m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `553 + m * (48 ±0) + p * (36 ±0)`
		//  Estimated: `6605 + m * (245 ±0) + p * (180 ±0)`
		// Minimum execution time: 26_385 nanoseconds.
		Weight::from_ref_time(27_964_035)
			.saturating_add(Weight::from_proof_size(6605))
			// Standard Error: 705
			.saturating_add(Weight::from_ref_time(17_445).saturating_mul(m.into()))
			// Standard Error: 688
			.saturating_add(Weight::from_ref_time(88_718).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_proof_size(245).saturating_mul(m.into()))
			.saturating_add(Weight::from_proof_size(180).saturating_mul(p.into()))
	}
	/// Storage: AllianceMotion Voting (r:1 w:1)
	/// Proof Skipped: AllianceMotion Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: AllianceMotion Members (r:1 w:0)
	/// Proof Skipped: AllianceMotion Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AllianceMotion Prime (r:1 w:0)
	/// Proof Skipped: AllianceMotion Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AllianceMotion ProposalOf (r:1 w:1)
	/// Proof Skipped: AllianceMotion ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// Storage: AllianceMotion Proposals (r:1 w:1)
	/// Proof Skipped: AllianceMotion Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_approved(b: u32, m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `812 + b * (1 ±0) + m * (64 ±0) + p * (40 ±0)`
		//  Estimated: `9715 + b * (5 ±0) + m * (330 ±0) + p * (200 ±0)`
		// Minimum execution time: 36_838 nanoseconds.
		Weight::from_ref_time(37_897_645)
			.saturating_add(Weight::from_proof_size(9715))
			// Standard Error: 102
			.saturating_add(Weight::from_ref_time(1_691).saturating_mul(b.into()))
			// Standard Error: 1_080
			.saturating_add(Weight::from_ref_time(16_517).saturating_mul(m.into()))
			// Standard Error: 1_053
			.saturating_add(Weight::from_ref_time(103_801).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_proof_size(5).saturating_mul(b.into()))
			.saturating_add(Weight::from_proof_size(330).saturating_mul(m.into()))
			.saturating_add(Weight::from_proof_size(200).saturating_mul(p.into()))
	}
	/// Storage: AllianceMotion Proposals (r:1 w:1)
	/// Proof Skipped: AllianceMotion Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: AllianceMotion Voting (r:0 w:1)
	/// Proof Skipped: AllianceMotion Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: AllianceMotion ProposalOf (r:0 w:1)
	/// Proof Skipped: AllianceMotion ProposalOf (max_values: None, max_size: None, mode: Measured)
	/// The range of component `p` is `[1, 100]`.
	fn disapprove_proposal(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `221 + p * (32 ±0)`
		//  Estimated: `1158 + p * (96 ±0)`
		// Minimum execution time: 12_696 nanoseconds.
		Weight::from_ref_time(15_035_512)
			.saturating_add(Weight::from_proof_size(1158))
			// Standard Error: 1_696
			.saturating_add(Weight::from_ref_time(74_633).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_proof_size(96).saturating_mul(p.into()))
	}
}
