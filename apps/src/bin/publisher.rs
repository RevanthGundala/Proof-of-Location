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

// This application demonstrates how to send an off-chain proof request
// to the Bonsai proving service and publish the received proofs directly
// to your deployed app contract.

use risc0_zkvm::serde::{from_slice, to_vec};
use alloy_primitives::{FixedBytes, U256};
use alloy_sol_types::{sol, SolInterface, SolValue};
use anyhow::{Context, Result};
use apps::{BonsaiProver, TxSender};
use clap::Parser;
use methods::IS_EVEN_ELF;
use tokio_util::compat::{FuturesAsyncReadCompatExt, TokioAsyncReadCompatExt};
use http_body_util::{BodyExt, Empty};
use hyper::{body::Bytes, Request, StatusCode};
use hyper_util::rt::TokioIo;
use std::{env, str};
use serde_json;
use serde::{Deserialize, Serialize};
use dotenv;
use tlsn_examples::request_notarization;
use tlsn_core::proof::TlsProof;
use tlsn_prover::tls::{Prover, ProverConfig};
use apps::{verify_tls_proof, find_ranges};
use axum::{
    routing::post,
    Router,
    extract::State,
    Json,
};
use tower_http::cors::CorsLayer;
use std::sync::Arc;
use std::sync::Mutex;
use tokio::sync::oneshot;

#[derive(Debug, Serialize, Deserialize)]
pub struct Input {
    start_long: f64,
    start_lat: f64,
    dest_long: f64,
    dest_lat: f64,
    distance: f64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PublicInput {
    dest_long: f64,
    dest_lat: f64,
    distance: f64,
}

#[derive(Deserialize)]
struct Payload {
    ip: String,
    longitude: String,
    latitude: String,
    distance: String,
}


sol! {
    interface IVerifier {
        function set(uint256 x, bytes32 post_state_digest, bytes calldata seal);
    }
}

/// Arguments of the publisher CLI.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Ethereum chain ID
    #[clap(long)]
    chain_id: u64,

    /// Ethereum Node endpoint.
    #[clap(long, env)]
    eth_wallet_private_key: String,

    /// Ethereum Node endpoint.
    #[clap(long)]
    rpc_url: String,

    /// Application's contract address on Ethereum
    #[clap(long)]
    contract: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let app = Router::new()
        .route("/api/prove", post(prove))
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    
    Ok(())
}

async fn prove(Json(payload): Json<Payload>) -> StatusCode {
    println!("Proving");
    let (ip, dest_long, dest_lat, distance) = (payload.ip, payload.longitude, payload.latitude, payload.distance);

    let start_long = geolocation::find(ip.as_str()).unwrap().longitude;
    let start_lat = geolocation::find(ip.as_str()).unwrap().latitude;

    // TODO: Find the distance between the user's location and the destination location
    // Send an off-chain proof request to the Bonsai proving service.
    let (tx, rx) = oneshot::channel();
    let input = Input {
        start_long: start_long.parse::<f64>().unwrap(),
        start_lat: start_lat.parse::<f64>().unwrap(),
        dest_long: dest_long.parse::<f64>().unwrap(),
        dest_lat: dest_lat.parse::<f64>().unwrap(),
        distance: distance.parse::<f64>().unwrap(),
    };
    let args = Args::parse();
    std::thread::spawn(move || {
        prove_and_send_transaction(args, input, tx);
    });

    match rx.await {
        Ok(_result) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

fn prove_and_send_transaction(
    args: Args,
    input: Input,
    tx: oneshot::Sender<(Vec<u8>, FixedBytes<32>, Vec<u8>)>,
) {
     // Create a new `TxSender`.
     let sender = TxSender::new(
        args.chain_id,
        &args.rpc_url,
        &args.eth_wallet_private_key,
        &args.contract,
    ).expect("Tx sender");
    let binding = bincode::serialize(&input).unwrap();
    let serialized_input = binding.as_slice();
    let public_input = PublicInput {
        dest_long: input.dest_long,
        dest_lat: input.dest_lat,
        distance: input.distance,
    };
    let (journal, post_state_digest, seal) = BonsaiProver::prove(IS_EVEN_ELF, serialized_input).unwrap();
    // assert!(public_input == from_slice(&journal).unwrap(), "Public input does not match");
    println!("Proved");
    let seal_clone = seal.clone();
    let x = U256::abi_decode(&journal, true).context("decoding journal data").unwrap();
    let calldata = IVerifier::IVerifierCalls::set(IVerifier::setCall {
        x,
        post_state_digest,
        seal: seal_clone,
    })
    .abi_encode();

    // Send the calldata to Ethereum.
    println!("Sending calldata");
    let runtime = tokio::runtime::Runtime::new().expect("failed to start new tokio runtime");
    runtime
        .block_on(sender.send(calldata))
        .expect("failed to send tx");

    tx.send((journal, post_state_digest, seal))
        .expect("failed to send over channel");
}
