#![no_main]

use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

pub fn main() {
    // TODO: Implement your guest code here

    loop {
        let input: u32 = env::read();
        println!("input1: {input}");
        env::commit(&input);
        env::pause(1);
    }
}
