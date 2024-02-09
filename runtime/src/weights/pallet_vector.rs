// This file is part of Substrate.

// Copyright (C) 2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for `pallet_vector`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-01-22, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `fedora`, CPU: `13th Gen Intel(R) Core(TM) i7-13700K`
//! EXECUTION: ``, WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: 1024

// Executed Command:
// ./target/release/avail-node
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_vector
// --extrinsic=*
// --heap-pages=4096
// --header=./HEADER-APACHE2
// --log=warn
// --output
// ./output/pallet_vector.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_vector`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_vector::WeightInfo for WeightInfo<T> {
	/// Storage: `Vector::WhitelistedDomains` (r:1 w:0)
	/// Proof: `Vector::WhitelistedDomains` (`max_values`: Some(1), `max_size`: Some(40002), added: 40497, mode: `MaxEncodedLen`)
	/// The range of component `l` is `[0, 102400]`.
	fn send_message_arbitrary_message(l: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `133`
		//  Estimated: `41487`
		// Minimum execution time: 8_026_000 picoseconds.
		Weight::from_parts(20_687_670, 0)
			.saturating_add(Weight::from_parts(0, 41487))
			// Standard Error: 0
			.saturating_add(Weight::from_parts(213, 0).saturating_mul(l.into()))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	/// Storage: `Vector::WhitelistedDomains` (r:1 w:0)
	/// Proof: `Vector::WhitelistedDomains` (`max_values`: Some(1), `max_size`: Some(40002), added: 40497, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn send_message_fungible_token() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `362`
		//  Estimated: `41487`
		// Minimum execution time: 43_340_000 picoseconds.
		Weight::from_parts(111_507_500, 0)
			.saturating_add(Weight::from_parts(0, 41487))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Vector::SyncCommitteePoseidons` (r:0 w:1)
	/// Proof: `Vector::SyncCommitteePoseidons` (`max_values`: None, `max_size`: Some(40), added: 2515, mode: `MaxEncodedLen`)
	fn set_poseidon_hash() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 6_628_000 picoseconds.
		Weight::from_parts(17_220_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Vector::Broadcasters` (r:1 w:1)
	/// Proof: `Vector::Broadcasters` (`max_values`: None, `max_size`: Some(36), added: 2511, mode: `MaxEncodedLen`)
	fn set_broadcaster() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `156`
		//  Estimated: `3501`
		// Minimum execution time: 9_785_000 picoseconds.
		Weight::from_parts(25_557_500, 0)
			.saturating_add(Weight::from_parts(0, 3501))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Vector::WhitelistedDomains` (r:0 w:1)
	/// Proof: `Vector::WhitelistedDomains` (`max_values`: Some(1), `max_size`: Some(40002), added: 40497, mode: `MaxEncodedLen`)
	fn set_whitelisted_domains() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 5_697_000 picoseconds.
		Weight::from_parts(14_802_500, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Vector::ConfigurationStorage` (r:0 w:1)
	/// Proof: `Vector::ConfigurationStorage` (`max_values`: Some(1), `max_size`: Some(10), added: 505, mode: `MaxEncodedLen`)
	fn set_configuration() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 5_476_000 picoseconds.
		Weight::from_parts(14_472_500, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Vector::SourceChainFrozen` (r:0 w:1)
	/// Proof: `Vector::SourceChainFrozen` (`max_values`: None, `max_size`: Some(5), added: 2480, mode: `MaxEncodedLen`)
	fn source_chain_froze() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 5_766_000 picoseconds.
		Weight::from_parts(15_497_500, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Vector::ConfigurationStorage` (r:1 w:0)
	/// Proof: `Vector::ConfigurationStorage` (`max_values`: Some(1), `max_size`: Some(10), added: 505, mode: `MaxEncodedLen`)
	/// Storage: `Vector::FunctionIds` (r:1 w:0)
	/// Proof: `Vector::FunctionIds` (`max_values`: Some(1), `max_size`: Some(65), added: 560, mode: `MaxEncodedLen`)
	/// Storage: `Vector::StepVerificationKey` (r:1 w:0)
	/// Proof: `Vector::StepVerificationKey` (`max_values`: Some(1), `max_size`: Some(10003), added: 10498, mode: `MaxEncodedLen`)
	/// Storage: `Vector::SyncCommitteePoseidons` (r:1 w:0)
	/// Proof: `Vector::SyncCommitteePoseidons` (`max_values`: None, `max_size`: Some(40), added: 2515, mode: `MaxEncodedLen`)
	/// Storage: `Vector::Head` (r:1 w:1)
	/// Proof: `Vector::Head` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Vector::Headers` (r:1 w:1)
	/// Proof: `Vector::Headers` (`max_values`: None, `max_size`: Some(40), added: 2515, mode: `MaxEncodedLen`)
	/// Storage: `Vector::ExecutionStateRoots` (r:1 w:1)
	/// Proof: `Vector::ExecutionStateRoots` (`max_values`: None, `max_size`: Some(40), added: 2515, mode: `MaxEncodedLen`)
	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Vector::Timestamps` (r:0 w:1)
	/// Proof: `Vector::Timestamps` (`max_values`: None, `max_size`: Some(16), added: 2491, mode: `MaxEncodedLen`)
	fn fulfill_call_step() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2485`
		//  Estimated: `11488`
		// Minimum execution time: 86_559_825_000 picoseconds.
		Weight::from_parts(86_605_948_000, 11488)
			.saturating_add(T::DbWeight::get().reads(8_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `Vector::ConfigurationStorage` (r:1 w:0)
	/// Proof: `Vector::ConfigurationStorage` (`max_values`: Some(1), `max_size`: Some(10), added: 505, mode: `MaxEncodedLen`)
	/// Storage: `Vector::FunctionIds` (r:1 w:0)
	/// Proof: `Vector::FunctionIds` (`max_values`: Some(1), `max_size`: Some(65), added: 560, mode: `MaxEncodedLen`)
	/// Storage: `Vector::RotateVerificationKey` (r:1 w:0)
	/// Proof: `Vector::RotateVerificationKey` (`max_values`: Some(1), `max_size`: Some(10003), added: 10498, mode: `MaxEncodedLen`)
	/// Storage: `Vector::Headers` (r:1 w:0)
	/// Proof: `Vector::Headers` (`max_values`: None, `max_size`: Some(40), added: 2515, mode: `MaxEncodedLen`)
	/// Storage: `Vector::SyncCommitteePoseidons` (r:1 w:1)
	/// Proof: `Vector::SyncCommitteePoseidons` (`max_values`: None, `max_size`: Some(40), added: 2515, mode: `MaxEncodedLen`)
	fn fulfill_call_rotate() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2958`
		//  Estimated: `11488`
		// Minimum execution time: 86_435_671_000 picoseconds.
		Weight::from_parts(86_578_476_000, 11488)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Vector::MessageStatus` (r:1 w:1)
	/// Proof: `Vector::MessageStatus` (`max_values`: None, `max_size`: Some(33), added: 2508, mode: `MaxEncodedLen`)
	/// Storage: `Vector::WhitelistedDomains` (r:1 w:0)
	/// Proof: `Vector::WhitelistedDomains` (`max_values`: Some(1), `max_size`: Some(40002), added: 40497, mode: `MaxEncodedLen`)
	/// Storage: `Vector::Broadcasters` (r:1 w:0)
	/// Proof: `Vector::Broadcasters` (`max_values`: None, `max_size`: Some(36), added: 2511, mode: `MaxEncodedLen`)
	/// Storage: `Vector::SourceChainFrozen` (r:1 w:0)
	/// Proof: `Vector::SourceChainFrozen` (`max_values`: None, `max_size`: Some(5), added: 2480, mode: `MaxEncodedLen`)
	/// Storage: `Vector::ExecutionStateRoots` (r:1 w:0)
	/// Proof: `Vector::ExecutionStateRoots` (`max_values`: None, `max_size`: Some(40), added: 2515, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn execute_fungible_token() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `394`
		//  Estimated: `41487`
		// Minimum execution time: 85_024_000 picoseconds.
		Weight::from_parts(217_517_500, 0)
			.saturating_add(Weight::from_parts(0, 41487))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Vector::MessageStatus` (r:1 w:1)
	/// Proof: `Vector::MessageStatus` (`max_values`: None, `max_size`: Some(33), added: 2508, mode: `MaxEncodedLen`)
	/// Storage: `Vector::WhitelistedDomains` (r:1 w:0)
	/// Proof: `Vector::WhitelistedDomains` (`max_values`: Some(1), `max_size`: Some(40002), added: 40497, mode: `MaxEncodedLen`)
	/// Storage: `Vector::Broadcasters` (r:1 w:0)
	/// Proof: `Vector::Broadcasters` (`max_values`: None, `max_size`: Some(36), added: 2511, mode: `MaxEncodedLen`)
	/// Storage: `Vector::SourceChainFrozen` (r:1 w:0)
	/// Proof: `Vector::SourceChainFrozen` (`max_values`: None, `max_size`: Some(5), added: 2480, mode: `MaxEncodedLen`)
	/// Storage: `Vector::ExecutionStateRoots` (r:1 w:0)
	/// Proof: `Vector::ExecutionStateRoots` (`max_values`: None, `max_size`: Some(40), added: 2515, mode: `MaxEncodedLen`)
	/// The range of component `l` is `[0, 102400]`.
	fn execute_arbitrary_message(l: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `216`
		//  Estimated: `41487`
		// Minimum execution time: 47_434_000 picoseconds.
		Weight::from_parts(122_728_200, 0)
			.saturating_add(Weight::from_parts(0, 41487))
			// Standard Error: 0
			.saturating_add(Weight::from_parts(3, 0).saturating_mul(l.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Vector::FunctionIds` (r:0 w:1)
	/// Proof: `Vector::FunctionIds` (`max_values`: Some(1), `max_size`: Some(65), added: 560, mode: `MaxEncodedLen`)
	fn set_function_ids() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 8_315_000 picoseconds.
		Weight::from_parts(9_217_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Vector::StepVerificationKey` (r:0 w:1)
	/// Proof: `Vector::StepVerificationKey` (`max_values`: Some(1), `max_size`: Some(10003), added: 10498, mode: `MaxEncodedLen`)
	fn set_step_verification_key() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 9_047_000 picoseconds.
		Weight::from_parts(9_398_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Vector::RotateVerificationKey` (r:0 w:1)
	/// Proof: `Vector::RotateVerificationKey` (`max_values`: Some(1), `max_size`: Some(10003), added: 10498, mode: `MaxEncodedLen`)
	fn set_rotate_verification_key() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 9_568_000 picoseconds.
		Weight::from_parts(9_979_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}
