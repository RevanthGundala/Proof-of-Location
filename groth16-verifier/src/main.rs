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

use groth16_verifier_methods::{GROTH16_VERIFIER_ELF, GROTH16_VERIFIER_ID};
use risc0_groth16::{ProofJson, PublicInputsJson, Verifier, VerifyingKeyJson};
use risc0_zkvm::{default_prover, sha::Digest, ExecutorEnv};
use sha2::{Digest as _, Sha256};

const PROOF: &str = include_str!("data/proof.json");
const PUBLIC_INPUTS: &str = include_str!("data/public.json");
const VERIFICATION_KEY: &str = include_str!("data/verification_key.json");

fn main() {
    // verification_key, proof and public witness generated by SnarkJS using Groth16 over BN254
    // (https://docs.circom.io/getting-started/proving-circuits/)
    println!("Starting");
    let proof: ProofJson = serde_json::from_str(PROOF).unwrap();
    let public_inputs = PublicInputsJson {
        values: serde_json::from_str(PUBLIC_INPUTS).unwrap(),
    };
    let verifying_key: VerifyingKeyJson = serde_json::from_str(VERIFICATION_KEY).unwrap();

    // we initialize a Groth16 verifier from the josn material generated by SnarkJS
    let verifier = Verifier::from_json(proof, public_inputs, verifying_key).unwrap();

    // groth16 proof verification
    verifier.verify().unwrap();
    // we configure an Executor with the groth16 verifier
    let env = ExecutorEnv::builder()
        .write(&verifier)
        .unwrap()
        .build()
        .unwrap();
    // we run the prover to generate a receipt of correct verification
    let receipt = default_prover().prove(env, GROTH16_VERIFIER_ELF).unwrap();
    println!("Here3");
    // we verify the final receipt
    receipt.verify(GROTH16_VERIFIER_ID).unwrap();
    println!("Here4");
    // we check that what committed into the journal matches with the input
    let committed_digest: Digest = receipt.journal.decode().unwrap();

    let mut hasher = Sha256::new();
    hasher.update(verifier.encoded_pvk);
    hasher.update(verifier.encoded_proof);
    hasher.update(verifier.encoded_prepared_inputs);
    let expected_digest = hasher.finalize();
    let expected_digest = Digest::try_from(expected_digest.as_slice()).unwrap();

    assert_eq!(committed_digest, expected_digest);

    println!("Verification: OK!");
}
