// Copyright 2023 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::io::Read;
use risc0_zkvm::guest::env;
use serde::{Deserialize, Serialize};
use alloy_sol_types::{sol, SolValue};
use risc0_zkvm::sha::rust_crypto::{Digest as _, Sha256};
use alloy_primitives::FixedBytes;

sol! {
    struct PublicInput {
        bytes32 start_long;
        bytes32 start_lat;
        bytes32 dest_long;
        bytes32 dest_lat;
        bytes32 distance;
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Input {
    start_long: f64,
    start_lat: f64,
    dest_long: f64,
    dest_lat: f64,
    distance: f64,
}

fn main() {
    // Read the input data for this application.
    let mut input_bytes = Vec::<u8>::new();
    env::stdin().read_to_end(&mut input_bytes).unwrap();
    // Decode and parse the input
    let input: Input = bincode::deserialize(&input_bytes).unwrap();

    // Commit the public input
    let start_long: FixedBytes<32> = FixedBytes::from_slice(Sha256::digest(input.start_long.to_le_bytes()).as_slice());
    let start_lat: FixedBytes<32> = FixedBytes::from_slice(Sha256::digest(input.start_lat.to_le_bytes()).as_slice());
    let dest_long: FixedBytes<32> = FixedBytes::from_slice(Sha256::digest(input.dest_long.to_le_bytes()).as_slice());
    let dest_lat: FixedBytes<32> = FixedBytes::from_slice(Sha256::digest(input.dest_lat.to_le_bytes()).as_slice());
    let distance: FixedBytes<32> = FixedBytes::from_slice(Sha256::digest(input.distance.to_le_bytes()).as_slice());

    let public_input = PublicInput {
        start_long,
        start_lat,
        dest_long,
        dest_lat,
        distance
    };

    let public_input = public_input.abi_encode();
    env::commit_slice(&public_input);

    let start_long = input.start_long;
    let start_lat = input.start_lat;
    let dest_long = input.dest_long;
    let dest_lat = input.dest_lat;
    let distance = input.distance;

    // Calculate if distance is within specified range using Haversine Formula
    assert!(is_within_distance(start_lat, start_long, dest_lat, dest_long, distance), "Distance is not within specified range");
}

fn haversine(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let earth_radius_miles = 3959.0;

    // Convert latitude and longitude from degrees to radians
    let lat1_rad = lat1.to_radians();
    let lon1_rad = lon1.to_radians();
    let lat2_rad = lat2.to_radians();
    let lon2_rad = lon2.to_radians();

    // Differences in coordinates
    let dlat = lat2_rad - lat1_rad;
    let dlon = lon2_rad - lon1_rad;

    // Haversine formula
    let a = (dlat / 2.0).sin().powi(2)
        + lat1_rad.cos() * lat2_rad.cos() * (dlon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

    // Calculate the distance
    earth_radius_miles * c
}

fn is_within_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64, max_distance: f64) -> bool {
    let distance = haversine(lat1, lon1, lat2, lon2);
    distance <= max_distance
}

