#![no_main]

use risc0_zkvm::guest::env;
risc0_zkvm::guest::entry!(main);
use std::str::FromStr;
use serde_json;


fn main() {
    // TODO: Implement your guest code he   re
    // read the input
    let input: String = env::read();
    let input: serde_json::Value = serde_json::Value::from_str(&input).unwrap();
    // TODO: do something with the input
    println!("Response: {}", input);

    // write public output to the journal
    env::commit(&input);
}
