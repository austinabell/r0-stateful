#![no_main]

use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let mut count = 0;
    loop {
        println!("pre read");
        let input: u32 = env::read();
        count += input;
        println!("input1: {input}");
        println!("count: {count}");
        env::commit(&input);
        env::pause(0);

        let input: u32 = env::read();
        count += input;
        println!("input2: {input}");
        println!("count: {count}");
        env::commit(&input);
        env::pause(0);
    }
}
