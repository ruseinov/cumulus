
//! Autogenerated weights for `pallet_referenda`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-03-06, STEPS: `2`, REPEAT: `1`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `cob`, CPU: `<UNKNOWN>`
//! EXECUTION: Some(Native), WASM-EXECUTION: Compiled, CHAIN: Some("collectives-polkadot-dev"), DB CACHE: 1024

// Executed Command:
// ./target/debug/polkadot-parachain
// benchmark
// pallet
// --chain=collectives-polkadot-dev
// --steps=2
// --repeat=1
// --pallet=pallet_referenda
// --extrinsic=*
// --execution=native
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./parachains/runtimes/collectives/collectives-polkadot/src/weights

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_referenda`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_referenda::WeightInfo for WeightInfo<T> {
	/// Storage: FellowshipCollective Members (r:1 w:0)
	/// Proof: FellowshipCollective Members (max_values: None, max_size: Some(42), added: 2517, mode: MaxEncodedLen)
	/// Storage: FellowshipReferenda ReferendumCount (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(155814), added: 158289, mode: MaxEncodedLen)
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:0 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	fn submit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `322`
		//  Estimated: `164275`
		// Minimum execution time: 283_000_000 picoseconds.
		Weight::from_parts(283_000_000, 0)
			.saturating_add(Weight::from_parts(0, 164275))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:2 w:2)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(155814), added: 158289, mode: MaxEncodedLen)
	fn place_decision_deposit_preparing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `430`
		//  Estimated: `321933`
		// Minimum execution time: 472_000_000 picoseconds.
		Weight::from_parts(472_000_000, 0)
			.saturating_add(Weight::from_parts(0, 321933))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: FellowshipReferenda DecidingCount (r:1 w:0)
	/// Proof: FellowshipReferenda DecidingCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: FellowshipReferenda TrackQueue (r:1 w:1)
	/// Proof: FellowshipReferenda TrackQueue (max_values: None, max_size: Some(812), added: 3287, mode: MaxEncodedLen)
	fn place_decision_deposit_queued() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1941`
		//  Estimated: `12121`
		// Minimum execution time: 547_000_000 picoseconds.
		Weight::from_parts(547_000_000, 0)
			.saturating_add(Weight::from_parts(0, 12121))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: FellowshipReferenda DecidingCount (r:1 w:0)
	/// Proof: FellowshipReferenda DecidingCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: FellowshipReferenda TrackQueue (r:1 w:1)
	/// Proof: FellowshipReferenda TrackQueue (max_values: None, max_size: Some(812), added: 3287, mode: MaxEncodedLen)
	fn place_decision_deposit_not_queued() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1982`
		//  Estimated: `12121`
		// Minimum execution time: 555_000_000 picoseconds.
		Weight::from_parts(555_000_000, 0)
			.saturating_add(Weight::from_parts(0, 12121))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: FellowshipReferenda DecidingCount (r:1 w:1)
	/// Proof: FellowshipReferenda DecidingCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: FellowshipCollective MemberCount (r:1 w:0)
	/// Proof: FellowshipCollective MemberCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:2 w:2)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(155814), added: 158289, mode: MaxEncodedLen)
	fn place_decision_deposit_passing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `833`
		//  Estimated: `328891`
		// Minimum execution time: 871_000_000 picoseconds.
		Weight::from_parts(871_000_000, 0)
			.saturating_add(Weight::from_parts(0, 328891))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: FellowshipReferenda DecidingCount (r:1 w:1)
	/// Proof: FellowshipReferenda DecidingCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: FellowshipCollective MemberCount (r:1 w:0)
	/// Proof: FellowshipCollective MemberCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	fn place_decision_deposit_failing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `643`
		//  Estimated: `11323`
		// Minimum execution time: 460_000_000 picoseconds.
		Weight::from_parts(460_000_000, 0)
			.saturating_add(Weight::from_parts(0, 11323))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	fn refund_decision_deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `381`
		//  Estimated: `4365`
		// Minimum execution time: 281_000_000 picoseconds.
		Weight::from_parts(281_000_000, 0)
			.saturating_add(Weight::from_parts(0, 4365))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	fn refund_submission_deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `199`
		//  Estimated: `4365`
		// Minimum execution time: 172_000_000 picoseconds.
		Weight::from_parts(172_000_000, 0)
			.saturating_add(Weight::from_parts(0, 4365))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:2 w:2)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(155814), added: 158289, mode: MaxEncodedLen)
	fn cancel() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `343`
		//  Estimated: `321933`
		// Minimum execution time: 334_000_000 picoseconds.
		Weight::from_parts(334_000_000, 0)
			.saturating_add(Weight::from_parts(0, 321933))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:2 w:2)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(155814), added: 158289, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: ParachainInfo ParachainId (r:1 w:0)
	/// Proof: ParachainInfo ParachainId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	/// Proof Skipped: PolkadotXcm SupportedVersion (max_values: None, max_size: None, mode: Measured)
	/// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	/// Proof Skipped: PolkadotXcm VersionDiscoveryQueue (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	/// Proof Skipped: PolkadotXcm SafeXcmVersion (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	/// Proof Skipped: ParachainSystem HostConfiguration (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	/// Proof Skipped: ParachainSystem PendingUpwardMessages (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: FellowshipReferenda MetadataOf (r:1 w:0)
	/// Proof: FellowshipReferenda MetadataOf (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	fn kill() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `581`
		//  Estimated: `342842`
		// Minimum execution time: 1_580_000_000 picoseconds.
		Weight::from_parts(1_580_000_000, 0)
			.saturating_add(Weight::from_parts(0, 342842))
			.saturating_add(T::DbWeight::get().reads(11))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: FellowshipReferenda TrackQueue (r:1 w:0)
	/// Proof: FellowshipReferenda TrackQueue (max_values: None, max_size: Some(812), added: 3287, mode: MaxEncodedLen)
	/// Storage: FellowshipReferenda DecidingCount (r:1 w:1)
	/// Proof: FellowshipReferenda DecidingCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	fn one_fewer_deciding_queue_empty() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `140`
		//  Estimated: `7756`
		// Minimum execution time: 203_000_000 picoseconds.
		Weight::from_parts(203_000_000, 0)
			.saturating_add(Weight::from_parts(0, 7756))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: FellowshipReferenda TrackQueue (r:1 w:1)
	/// Proof: FellowshipReferenda TrackQueue (max_values: None, max_size: Some(812), added: 3287, mode: MaxEncodedLen)
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: FellowshipCollective MemberCount (r:1 w:0)
	/// Proof: FellowshipCollective MemberCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:2 w:2)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(155814), added: 158289, mode: MaxEncodedLen)
	fn one_fewer_deciding_failing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3991`
		//  Estimated: `329689`
		// Minimum execution time: 1_025_000_000 picoseconds.
		Weight::from_parts(1_025_000_000, 0)
			.saturating_add(Weight::from_parts(0, 329689))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: FellowshipReferenda TrackQueue (r:1 w:1)
	/// Proof: FellowshipReferenda TrackQueue (max_values: None, max_size: Some(812), added: 3287, mode: MaxEncodedLen)
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: FellowshipCollective MemberCount (r:1 w:0)
	/// Proof: FellowshipCollective MemberCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:2 w:2)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(155814), added: 158289, mode: MaxEncodedLen)
	fn one_fewer_deciding_passing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3991`
		//  Estimated: `329689`
		// Minimum execution time: 1_076_000_000 picoseconds.
		Weight::from_parts(1_076_000_000, 0)
			.saturating_add(Weight::from_parts(0, 329689))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: FellowshipReferenda TrackQueue (r:1 w:1)
	/// Proof: FellowshipReferenda TrackQueue (max_values: None, max_size: Some(812), added: 3287, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(155814), added: 158289, mode: MaxEncodedLen)
	fn nudge_referendum_requeued_insertion() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3589`
		//  Estimated: `167921`
		// Minimum execution time: 621_000_000 picoseconds.
		Weight::from_parts(621_000_000, 0)
			.saturating_add(Weight::from_parts(0, 167921))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: FellowshipReferenda TrackQueue (r:1 w:1)
	/// Proof: FellowshipReferenda TrackQueue (max_values: None, max_size: Some(812), added: 3287, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(155814), added: 158289, mode: MaxEncodedLen)
	fn nudge_referendum_requeued_slide() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3542`
		//  Estimated: `167921`
		// Minimum execution time: 643_000_000 picoseconds.
		Weight::from_parts(643_000_000, 0)
			.saturating_add(Weight::from_parts(0, 167921))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: FellowshipReferenda DecidingCount (r:1 w:0)
	/// Proof: FellowshipReferenda DecidingCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: FellowshipReferenda TrackQueue (r:1 w:1)
	/// Proof: FellowshipReferenda TrackQueue (max_values: None, max_size: Some(812), added: 3287, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(155814), added: 158289, mode: MaxEncodedLen)
	fn nudge_referendum_queued() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3528`
		//  Estimated: `171400`
		// Minimum execution time: 704_000_000 picoseconds.
		Weight::from_parts(704_000_000, 0)
			.saturating_add(Weight::from_parts(0, 171400))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: FellowshipReferenda DecidingCount (r:1 w:0)
	/// Proof: FellowshipReferenda DecidingCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: FellowshipReferenda TrackQueue (r:1 w:1)
	/// Proof: FellowshipReferenda TrackQueue (max_values: None, max_size: Some(812), added: 3287, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(155814), added: 158289, mode: MaxEncodedLen)
	fn nudge_referendum_not_queued() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3583`
		//  Estimated: `171400`
		// Minimum execution time: 695_000_000 picoseconds.
		Weight::from_parts(695_000_000, 0)
			.saturating_add(Weight::from_parts(0, 171400))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(155814), added: 158289, mode: MaxEncodedLen)
	fn nudge_referendum_no_deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `295`
		//  Estimated: `163644`
		// Minimum execution time: 259_000_000 picoseconds.
		Weight::from_parts(259_000_000, 0)
			.saturating_add(Weight::from_parts(0, 163644))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(155814), added: 158289, mode: MaxEncodedLen)
	fn nudge_referendum_preparing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `343`
		//  Estimated: `163644`
		// Minimum execution time: 260_000_000 picoseconds.
		Weight::from_parts(260_000_000, 0)
			.saturating_add(Weight::from_parts(0, 163644))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	fn nudge_referendum_timed_out() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `240`
		//  Estimated: `4365`
		// Minimum execution time: 184_000_000 picoseconds.
		Weight::from_parts(184_000_000, 0)
			.saturating_add(Weight::from_parts(0, 4365))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: FellowshipReferenda DecidingCount (r:1 w:1)
	/// Proof: FellowshipReferenda DecidingCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: FellowshipCollective MemberCount (r:1 w:0)
	/// Proof: FellowshipCollective MemberCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(155814), added: 158289, mode: MaxEncodedLen)
	fn nudge_referendum_begin_deciding_failing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `611`
		//  Estimated: `170602`
		// Minimum execution time: 445_000_000 picoseconds.
		Weight::from_parts(445_000_000, 0)
			.saturating_add(Weight::from_parts(0, 170602))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: FellowshipReferenda DecidingCount (r:1 w:1)
	/// Proof: FellowshipReferenda DecidingCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: FellowshipCollective MemberCount (r:1 w:0)
	/// Proof: FellowshipCollective MemberCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(155814), added: 158289, mode: MaxEncodedLen)
	fn nudge_referendum_begin_deciding_passing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `746`
		//  Estimated: `170602`
		// Minimum execution time: 625_000_000 picoseconds.
		Weight::from_parts(625_000_000, 0)
			.saturating_add(Weight::from_parts(0, 170602))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: FellowshipCollective MemberCount (r:1 w:0)
	/// Proof: FellowshipCollective MemberCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(155814), added: 158289, mode: MaxEncodedLen)
	fn nudge_referendum_begin_confirming() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `799`
		//  Estimated: `167123`
		// Minimum execution time: 623_000_000 picoseconds.
		Weight::from_parts(623_000_000, 0)
			.saturating_add(Weight::from_parts(0, 167123))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: FellowshipCollective MemberCount (r:1 w:0)
	/// Proof: FellowshipCollective MemberCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(155814), added: 158289, mode: MaxEncodedLen)
	fn nudge_referendum_end_confirming() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `782`
		//  Estimated: `167123`
		// Minimum execution time: 580_000_000 picoseconds.
		Weight::from_parts(580_000_000, 0)
			.saturating_add(Weight::from_parts(0, 167123))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: FellowshipCollective MemberCount (r:1 w:0)
	/// Proof: FellowshipCollective MemberCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(155814), added: 158289, mode: MaxEncodedLen)
	fn nudge_referendum_continue_not_confirming() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `799`
		//  Estimated: `167123`
		// Minimum execution time: 595_000_000 picoseconds.
		Weight::from_parts(595_000_000, 0)
			.saturating_add(Weight::from_parts(0, 167123))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: FellowshipCollective MemberCount (r:1 w:0)
	/// Proof: FellowshipCollective MemberCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(155814), added: 158289, mode: MaxEncodedLen)
	fn nudge_referendum_continue_confirming() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `803`
		//  Estimated: `167123`
		// Minimum execution time: 556_000_000 picoseconds.
		Weight::from_parts(556_000_000, 0)
			.saturating_add(Weight::from_parts(0, 167123))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: FellowshipCollective MemberCount (r:1 w:0)
	/// Proof: FellowshipCollective MemberCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:2 w:2)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(155814), added: 158289, mode: MaxEncodedLen)
	/// Storage: Scheduler Lookup (r:1 w:1)
	/// Proof: Scheduler Lookup (max_values: None, max_size: Some(48), added: 2523, mode: MaxEncodedLen)
	fn nudge_referendum_approved() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `803`
		//  Estimated: `328925`
		// Minimum execution time: 704_000_000 picoseconds.
		Weight::from_parts(704_000_000, 0)
			.saturating_add(Weight::from_parts(0, 328925))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:1)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: FellowshipCollective MemberCount (r:1 w:0)
	/// Proof: FellowshipCollective MemberCount (max_values: None, max_size: Some(14), added: 2489, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(155814), added: 158289, mode: MaxEncodedLen)
	fn nudge_referendum_rejected() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `799`
		//  Estimated: `167123`
		// Minimum execution time: 615_000_000 picoseconds.
		Weight::from_parts(615_000_000, 0)
			.saturating_add(Weight::from_parts(0, 167123))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:0)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: Preimage StatusFor (r:1 w:0)
	/// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	/// Storage: FellowshipReferenda MetadataOf (r:0 w:1)
	/// Proof: FellowshipReferenda MetadataOf (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	fn set_some_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `384`
		//  Estimated: `7921`
		// Minimum execution time: 262_000_000 picoseconds.
		Weight::from_parts(262_000_000, 0)
			.saturating_add(Weight::from_parts(0, 7921))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: FellowshipReferenda ReferendumInfoFor (r:1 w:0)
	/// Proof: FellowshipReferenda ReferendumInfoFor (max_values: None, max_size: Some(900), added: 3375, mode: MaxEncodedLen)
	/// Storage: FellowshipReferenda MetadataOf (r:1 w:1)
	/// Proof: FellowshipReferenda MetadataOf (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	fn clear_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `317`
		//  Estimated: `7882`
		// Minimum execution time: 223_000_000 picoseconds.
		Weight::from_parts(223_000_000, 0)
			.saturating_add(Weight::from_parts(0, 7882))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
