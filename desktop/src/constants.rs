use chip8_core::constants;

const SCALE: u32 = 15;
pub const WINDOW_WIDTH: u32 = (constants::SCREEN_WIDTH as u32) * SCALE;
pub const WINDOW_HEIGHT: u32 = (constants::SCREEN_HEIGHT as u32) * SCALE;
