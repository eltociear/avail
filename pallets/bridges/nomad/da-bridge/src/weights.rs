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

//! Autogenerated weights for `nomad_da_bridge`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-09-14, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ip-172-31-12-189`, CPU: `Intel(R) Xeon(R) Platinum 8175M CPU @ 2.50GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
// ./target/release/avail-node
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --log=warn
// --template=./.maintain/frame-weight-template.hbs
// --header=./HEADER-APACHE2
// --pallet=nomad_da_bridge
// --extrinsic=*
// --output=./output/nomad_da_bridge_weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for `nomad_da_bridge`.
pub trait WeightInfo {
	fn try_dispatch_data_root() -> Weight;
}

/// Weights for `nomad_da_bridge` using the Avail node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `System::BlockHash` (r:1 w:0)
	/// Proof: `System::BlockHash` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	/// Storage: `NomadHome::Base` (r:1 w:0)
	/// Proof: `NomadHome::Base` (`max_values`: Some(1), `max_size`: Some(57), added: 552, mode: `MaxEncodedLen`)
	/// Storage: `NomadHome::Nonces` (r:1 w:1)
	/// Proof: `NomadHome::Nonces` (`max_values`: None, `max_size`: Some(16), added: 2491, mode: `MaxEncodedLen`)
	/// Storage: `NomadHome::Tree` (r:1 w:1)
	/// Proof: `NomadHome::Tree` (`max_values`: Some(1), `max_size`: Some(1028), added: 1523, mode: `MaxEncodedLen`)
	/// Storage: `NomadHome::IndexToRoot` (r:0 w:1)
	/// Proof: `NomadHome::IndexToRoot` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	/// Storage: `NomadHome::RootToIndex` (r:0 w:1)
	/// Proof: `NomadHome::RootToIndex` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	fn try_dispatch_data_root() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1269`
		//  Estimated: `3509`
		// Minimum execution time: 132_465_000 picoseconds.
		Weight::from_parts(137_751_000, 3509)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
}

// For backwards compatibility and tests.
impl WeightInfo for () {
	/// Storage: `System::BlockHash` (r:1 w:0)
	/// Proof: `System::BlockHash` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	/// Storage: `NomadHome::Base` (r:1 w:0)
	/// Proof: `NomadHome::Base` (`max_values`: Some(1), `max_size`: Some(57), added: 552, mode: `MaxEncodedLen`)
	/// Storage: `NomadHome::Nonces` (r:1 w:1)
	/// Proof: `NomadHome::Nonces` (`max_values`: None, `max_size`: Some(16), added: 2491, mode: `MaxEncodedLen`)
	/// Storage: `NomadHome::Tree` (r:1 w:1)
	/// Proof: `NomadHome::Tree` (`max_values`: Some(1), `max_size`: Some(1028), added: 1523, mode: `MaxEncodedLen`)
	/// Storage: `NomadHome::IndexToRoot` (r:0 w:1)
	/// Proof: `NomadHome::IndexToRoot` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	/// Storage: `NomadHome::RootToIndex` (r:0 w:1)
	/// Proof: `NomadHome::RootToIndex` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	fn try_dispatch_data_root() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1269`
		//  Estimated: `3509`
		// Minimum execution time: 132_465_000 picoseconds.
		Weight::from_parts(137_751_000, 3509)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
}