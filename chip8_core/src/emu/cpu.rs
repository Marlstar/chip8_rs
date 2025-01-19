use crate::emu::SCREEN_WIDTH;

use super::Opcode;

impl super::Emu {
    pub fn tick(&mut self) {
        let op = Self::decode_opcode(self.fetch_opcode());
        self.execute_opcode(op);
    }

    fn fetch_opcode(&mut self) -> u16 {
        let higher_byte = self.ram[self.pc as usize] as u16;
        let lower_byte = self.ram[(self.pc + 1) as usize] as u16;
        // Combine the two u8's into one u16 opcode
        let opcode = (higher_byte << 8) | lower_byte;
        self.skip();
        return opcode;
    }


    fn decode_opcode(opcode: u16) -> Opcode {
        return Opcode::new(opcode);
    }

    fn execute_opcode(&mut self, opcode: Opcode) {
        use super::Opcode::*;
        match opcode {
            Nop => return,

            ClearScreen => {
                self.screen = [false; super::SCREEN_WIDTH * super::SCREEN_HEIGHT];
            },

            Return => {
                let ret = self.pop();
                self.pc = ret;
            },

            Jump(addr) => {
                self.pc = addr;
            },

            Call(addr) => {
                self.push(self.pc);
                self.pc = addr;
            }

            SkipIfValEQ(reg, val) => {
                if self.v_reg[reg] == val {
                    self.skip();
                }
            },

            SkipIfValNE(reg, val) => {
                if self.v_reg[reg] != val {
                    self.skip();
                }
            },

            SkipIfRegEQ(reg1, reg2) => {
                if self.v_reg[reg1] == self.v_reg[reg2] {
                    self.skip();
                }
            }

            SetToVal(reg, val) => {
                self.v_reg[reg] = val;
            },

            AddVal(reg, val) => {
                self.v_reg[reg] = self.v_reg[reg].wrapping_add(val);
            },

            SetToReg(reg, src) => {
                self.v_reg[reg] = self.v_reg[src];
            },

            BitwiseOr(reg1, reg2) => {
                self.v_reg[reg1] |= self.v_reg[reg2];
            },

            BitwiseAnd(reg1, reg2) => {
                self.v_reg[reg1] &= self.v_reg[reg2];
            },

            BitwiseXor(reg1, reg2) => {
                self.v_reg[reg1] ^= self.v_reg[reg2];
            },

            AddReg(reg1, reg2) => {
                let (val, carry) = self.v_reg[reg1].overflowing_add(self.v_reg[reg2]);
                self.v_reg[reg1] = val;
                self.v_reg[0xF] = if carry { 1 } else { 0 };
            },

            SubReg(reg1, reg2) => {
                let (val, borrow) = self.v_reg[reg1].overflowing_sub(self.v_reg[reg2]);
                self.v_reg[reg1] = val;
                self.v_reg[0xF] = if borrow { 0 } else { 1 };
            },

            ShiftRight(reg) => {
                let lsb = self.v_reg[reg] & 1; // Dropped bit
                self.v_reg[reg] >>= 1;
                self.v_reg[0xF] = lsb;
            },

            SubFromReg(reg1, reg2) => {
                let (val, borrow) = self.v_reg[reg2].overflowing_sub(self.v_reg[reg1]);
                self.v_reg[reg1] = val;
                self.v_reg[0xF] = if borrow { 0 } else { 1 };
            },

            ShiftLeft(reg) => {
                let msb = (self.v_reg[reg] >> 7) & 1; // Overflowed bit
                self.v_reg[reg] <<= 1;
                self.v_reg[0xF] = msb;
            },


            SkipIfRegNE(reg1, reg2) => {
                if self.v_reg[reg1] != self.v_reg[reg2] {
                    self.skip();
                }
            },

            SetIndex(addr) => {
                self.i_reg = addr;
            },

            JumpV0Distance(distance) => {
                self.pc = (self.v_reg[0] as u16) + distance;
            },

            Rand(reg, num) => {
                let rng: u8 = rand::random();
                self.v_reg[reg] = rng & num;
            },

            DrawSprite(x_coord, y_coord, height) => {
                let mut flipped = false;

                for row in 0..height {
                    let addr = self.i_reg + row as u16; // Sprite data address for the row
                    let pixels = self.ram[addr as usize]; // Sprite data for the row

                    for col in 0..8 { // Sprites always have 8 colums
                        // If the sprite's pixel is a 1, then the screen pixel will flip
                        // If the screen pixel flips from 0 -> 1, VF is not set
                        // If the screen pixel flips from 1 -> 0, VF is set

                        let mask = 0b1000_0000 >> col; // mask for the current bit
                        let sprite_pixel = pixels & mask;
                        if sprite_pixel != 0 { // If the pixel is on
                            // Position of pixel on the screen. Overflows wrap using the modulo.
                            let x = (x_coord + col) as usize % super::SCREEN_WIDTH;
                            let y = (y_coord + row as u16) as usize % super::SCREEN_HEIGHT;
                            // Memory index for the screen pixel
                            let index = SCREEN_WIDTH * y + x;
                            let screen_pixel = self.screen[index];
                            if screen_pixel {
                                self.screen[index] = false;
                                flipped = true;
                            } else {
                                self.screen[index] = true;
                            }
                        }
                    }

                    self.v_reg[0xF] = if flipped { 1 } else { 0 };
                }
            },

            SkipIfKeyPressed(reg) => {
                let index = self.v_reg[reg] as usize;
                let key = self.keys[index];
                if key { self.skip(); }
            },

            SkipIfKeyNotPressed(reg) => {
                let index = self.v_reg[reg] as usize;
                let key = self.keys[index];
                if !key { self.skip(); }
            },

            GetDelayTimer(reg) => {
                self.v_reg[reg] = self.dt;
            }

            WaitKey(reg) => {
                let mut press = false;
                for i in 0..super::NUM_KEYS {
                    if self.keys[i] {
                        self.v_reg[reg] = i as u8;
                        press = true;
                        break;
                    }
                }
                if !press {
                    // Redo the instruction
                    self.pc -= 2;
                }
            },

            SetDelayTimer(reg) => {
                self.dt = self.v_reg[reg];
            },

            SetSoundTimer(reg) => {
                self.st = self.v_reg[reg];
            },

            IncrementI(reg) => {
                self.i_reg = self.i_reg.wrapping_add(self.v_reg[reg] as u16)
            },

            LoadFontChar(reg) => {
                let character = self.v_reg[reg] as u16;
                // Move the `I` register to the start address of the font
                // and then increment it by the character offset
                // multiplied by the amouunt of rows in a character (5)
                self.i_reg = super::FONT_START_ADDR + character * 5;
            },

            BCD(reg) => {
                let num = self.v_reg[reg];

                let ones = num % 10;
                let tens = (num - ones) % 100 / 10;
                let hundreds = (num - ones - (tens*10)) / 100;

                let base_addr = self.i_reg as usize;
                self.ram[base_addr] = hundreds;
                self.ram[base_addr + 1] = tens;
                self.ram[base_addr + 2] = ones;
            },

            LoadIntoRam(reg) => {
                let i = self.i_reg as usize;
                for idx in 0..=reg {
                    self.ram[i + idx] = self.v_reg[idx];
                }
            },

            LoadFromRam(reg) => {
                let i = self.i_reg as usize;
                for idx in 0..=reg {
                    self.v_reg[idx] = self.ram[i + idx];
                }
            },
        }
    }

    pub(super) fn skip(&mut self) {
        self.pc += 2;
    }
}
