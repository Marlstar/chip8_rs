use crate::constants::*;
mod stack;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Emu {
    pc: u16, // Program counter
    ram: [u8; RAM_SIZE], // Main RAM
    v_reg: [u8; NUM_REGS], // Main registers
    i_reg: u16, // Used for indexing into RAM for reading/writing
    sp: u16, // Stack pointer
    stack: [u16; STACK_SIZE], // The stack itself
    keys: [bool; NUM_KEYS], // Keys
    dt: u8, // Delay timer
    st: u8, // Sound timer

    screen: [bool; SCREEN_WIDTH * SCREEN_HEIGHT], // Screen data
}
impl Default for Emu {
    fn default() -> Self {
        return Self::new();
    }
}
impl Emu {
    pub fn new() -> Self {
        Self {
            pc: START_ADDR,
            ram: [0; RAM_SIZE],
            screen: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
            v_reg: [0; NUM_REGS],
            i_reg: 0,
            sp: 0,
            stack: [0; STACK_SIZE],
            keys: [false; NUM_KEYS],
            dt: 0,
            st: 0,
        }
    }
}
