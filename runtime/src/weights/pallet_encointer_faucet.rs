
//! Autogenerated weights for `pallet_encointer_faucet`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-15, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `Fabian-iPhone.localdomain`, CPU: `<UNKNOWN>`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/release/encointer-node-notee
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_encointer_faucet
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=runtime/src/weights/pallet_encointer_faucet.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_encointer_faucet`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_encointer_faucet::WeightInfo for WeightInfo<T> {
	/// Storage: EncointerCommunities CommunityIdentifiers (r:1 w:0)
	/// Proof: EncointerCommunities CommunityIdentifiers (max_values: Some(1), max_size: Some(90002), added: 90497, mode: MaxEncodedLen)
	/// Storage: EncointerFaucet Faucets (r:1 w:1)
	/// Proof: EncointerFaucet Faucets (max_values: None, max_size: Some(9372), added: 11847, mode: MaxEncodedLen)
	/// Storage: EncointerFaucet ReserveAmount (r:1 w:0)
	/// Proof: EncointerFaucet ReserveAmount (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: EncointerReputationCommitments CurrentPurposeId (r:1 w:1)
	/// Proof: EncointerReputationCommitments CurrentPurposeId (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: EncointerReputationCommitments Purposes (r:0 w:1)
	/// Proof: EncointerReputationCommitments Purposes (max_values: None, max_size: Some(138), added: 2613, mode: MaxEncodedLen)
	fn create_faucet() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `195`
		//  Estimated: `91487`
		// Minimum execution time: 214_000_000 picoseconds.
		Weight::from_parts(228_000_000, 0)
			.saturating_add(Weight::from_parts(0, 91487))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: EncointerFaucet Faucets (r:1 w:0)
	/// Proof: EncointerFaucet Faucets (max_values: None, max_size: Some(9372), added: 11847, mode: MaxEncodedLen)
	/// Storage: EncointerReputationCommitments Purposes (r:1 w:0)
	/// Proof: EncointerReputationCommitments Purposes (max_values: None, max_size: Some(138), added: 2613, mode: MaxEncodedLen)
	/// Storage: EncointerCeremonies ParticipantReputation (r:1 w:0)
	/// Proof Skipped: EncointerCeremonies ParticipantReputation (max_values: None, max_size: None, mode: Measured)
	/// Storage: EncointerReputationCommitments Commitments (r:1 w:1)
	/// Proof: EncointerReputationCommitments Commitments (max_values: None, max_size: Some(102), added: 2577, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn drip() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `668`
		//  Estimated: `12837`
		// Minimum execution time: 197_000_000 picoseconds.
		Weight::from_parts(199_000_000, 0)
			.saturating_add(Weight::from_parts(0, 12837))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: EncointerFaucet Faucets (r:1 w:1)
	/// Proof: EncointerFaucet Faucets (max_values: None, max_size: Some(9372), added: 11847, mode: MaxEncodedLen)
	/// Storage: Balances Reserves (r:1 w:0)
	/// Proof: Balances Reserves (max_values: None, max_size: Some(3122), added: 5597, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn dissolve_faucet() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `331`
		//  Estimated: `12837`
		// Minimum execution time: 173_000_000 picoseconds.
		Weight::from_parts(181_000_000, 0)
			.saturating_add(Weight::from_parts(0, 12837))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: EncointerFaucet Faucets (r:1 w:1)
	/// Proof: EncointerFaucet Faucets (max_values: None, max_size: Some(9372), added: 11847, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Balances Reserves (r:1 w:0)
	/// Proof: Balances Reserves (max_values: None, max_size: Some(3122), added: 5597, mode: MaxEncodedLen)
	fn close_faucet() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `331`
		//  Estimated: `12837`
		// Minimum execution time: 175_000_000 picoseconds.
		Weight::from_parts(183_000_000, 0)
			.saturating_add(Weight::from_parts(0, 12837))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: EncointerFaucet ReserveAmount (r:0 w:1)
	/// Proof: EncointerFaucet ReserveAmount (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	fn set_reserve_amount() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 29_000_000 picoseconds.
		Weight::from_parts(30_000_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
