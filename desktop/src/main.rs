use chip8_core::Emu;
use std::env;
use std::io::Read;

fn main() {
    let args: Vec<_> = env::args().collect();
    // If rom path not specified, or too many args are supplied
    if args.len() != 2 {
        println!("Usage: cargo run path/to/rom");
        return;
    }

    let mut chip8 = Emu::new();

    { // Load the rom
        let mut rom = std::fs::File::open(&args[1]).expect("Unable to open rom");
        let mut buffer = Vec::new();
        rom.read_to_end(&mut buffer).unwrap();
        chip8.load_rom(&buffer);
    }
}
