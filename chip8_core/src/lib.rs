#![allow(dead_code, clippy::needless_return)]

mod emu;
pub use emu::Emu;

pub mod constants;

pub mod resources;
pub use resources::font;
