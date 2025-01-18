use crate::constants;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Emu {
    pc: u16, // Program counter
    ram: [u8; constants::RAM_SIZE], // Main RAM
    v_reg: [u8; constants::NUM_REGS], // Main registers
    i_reg: u16, // Used for indexing into RAM for reading/writing
    sp: u16, // Stack pointer
    stack: [u16; constants::STACK_SIZE], // The stack itself
    keys: [bool; constants::NUM_KEYS], // Keys
    dt: u8, // Delay timer
    st: u8, // Sound timer

    screen: [bool; constants::SCREEN_WIDTH * constants::SCREEN_HEIGHT], // Screen data
}
