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

use ethers::types::TransactionReceipt;
use alloy_sol_types::{sol, SolInterface, SolValue};
use anyhow::{Context, Result};
use apps::{BonsaiProver, TxSender};
use clap::Parser;
use methods::IS_LOCATED_ELF;
use std::str;
use serde::{Deserialize, Serialize};
use axum::{
    routing::post,
    Router,
    Json,
    http::StatusCode,
    response::IntoResponse,
};
use tower_http::cors::CorsLayer;
use tokio::sync::oneshot;

#[derive(Serialize)]
struct ProveResponse {
    tx_receipt: TransactionReceipt,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Input {
    start_long: f64,
    start_lat: f64,
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
        function verifyLocation(bytes32 start_long, bytes32 start_lat, bytes32 dest_long, bytes32 dest_lat, bytes32 distance, bytes32 post_state_digest, bytes calldata seal);
    }

    struct PublicInput {
        bytes32 start_long;
        bytes32 start_lat;
        bytes32 dest_long;
        bytes32 dest_lat;
        bytes32 distance;
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

async fn prove(Json(payload): Json<Payload>) -> impl IntoResponse {
    println!("Proving");
    let (ip, dest_long, dest_lat, distance) = (payload.ip, payload.longitude, payload.latitude, payload.distance);

    let start_long = geolocation::find(ip.as_str()).unwrap().longitude;
    let start_lat = geolocation::find(ip.as_str()).unwrap().latitude;

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
        Ok(result) => {
            println!("Transaction hash: {:?}", result.transaction_hash);
            let response_body = ProveResponse {
                tx_receipt: result,
            };
            (StatusCode::OK, Json(response_body))
        },
        Err(_) => {
            let response_body = ProveResponse {
                tx_receipt: TransactionReceipt::default(),
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response_body))
        }
    }
}

fn prove_and_send_transaction(
    args: Args,
    input: Input,
    tx: oneshot::Sender<TransactionReceipt>,
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
    let (journal, post_state_digest, seal) = BonsaiProver::prove(IS_LOCATED_ELF, serialized_input).unwrap();
    println!("Proved");
    let seal_clone = seal.clone();
    let public_input = PublicInput::abi_decode(&journal, true).context("decoding journal data").unwrap();
    let calldata = IVerifier::IVerifierCalls::verifyLocation(IVerifier::verifyLocationCall {
        start_long: public_input.start_long,
        start_lat: public_input.start_lat,
        dest_long: public_input.dest_long,
        dest_lat: public_input.dest_lat,
        distance: public_input.distance,
        post_state_digest,
        seal: seal_clone,
    })
    .abi_encode();

    // Send the calldata to Ethereum.
    println!("Sending calldata");
    let runtime = tokio::runtime::Runtime::new().expect("failed to start new tokio runtime");
    let tx_result = runtime
        .block_on(sender.send(calldata))
        .expect("failed to send tx");
    let Some(tx_result) = tx_result else {
        println!("Transaction failed");
        return;
    };
    tx.send(tx_result).expect("failed to send over channel");
}
