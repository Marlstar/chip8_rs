use chip8_core::Emu;
use desktop::constants::*;
use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    // If rom path not specified, or too many args are supplied
    if args.len() != 2 {
        println!("Usage: cargo run path/to/rom");
        return;
    }
}
