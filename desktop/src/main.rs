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

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Chip-8 Emulator", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    canvas.clear();
    canvas.present();
}
