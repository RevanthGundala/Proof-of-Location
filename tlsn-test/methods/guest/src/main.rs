#![no_main]
// If you want to try std support, also update the guest Cargo.toml file

use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);


fn main() {
    // TODO: Implement your guest code here

    // read the input
    let input: u32 = env::read();
    // TODO: do something with the input
    println!("Input: {}", input);

    // write public output to the journal
    env::commit(&input);
}
