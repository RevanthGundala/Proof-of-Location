// Copyright 2024 RISC Zero, Inc.
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

#![no_main]

use risc0_groth16::Verifier;
use risc0_zkvm::{guest::env, sha::Digest};
use sha2::{Digest as _, Sha256};

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let verifier: Verifier = env::read();
    verifier.verify().unwrap();
    let mut hasher = Sha256::new();
    println!("{:?}", verifier.encoded_pvk);
    println!("{:?}", verifier.encoded_proof);
    println!("{:?}", verifier.encoded_prepared_inputs);
    hasher.update(verifier.encoded_pvk);
    hasher.update(verifier.encoded_proof);
    hasher.update(verifier.encoded_prepared_inputs);
    let digest = hasher.finalize();
    let digest = Digest::try_from(digest.as_slice()).unwrap();
    env::commit(&digest);
}