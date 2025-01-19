pub enum Opcode {
    /// Do nothing
    /// Opcode: `0000`
    Nop,

    /// Clear the screen
    ///
    /// Opcode: `00E0`
    ClearScreen,

    /// Return from subroutine
    ///
    /// Opcode: `00EE`
    Return,

    /// Jump to address `0xNNN`
    ///
    /// Arguments: `(NNN)`
    /// Opcode: `1NNN`
    Jump(u16),

    /// Call subroutine at `0xNNN`
    /// Adds current PC to the stack as a return pointer for later
    ///
    /// Arguments: `(NNN)`
    /// Opcode: `2NNN`
    Call(u16),

    /// Skip an instruction if `VX == 0xNN`
    ///
    /// Arguments: `(VX, NN)`
    /// Opcode: `3XNN`
    SkipIfValEQ(usize, u8),

    /// Skip an instruction if `VX != 0xNN`
    ///
    /// Arguments: `(VX, NN)`
    /// Opcode: `4XNN`
    SkipIfValNE(usize, u8),

    /// Skip an instruction if `VX == VY`
    ///
    /// Arguments: `(VX, VY)`
    /// Opcode: `5XY0`
    SkipIfRegEQ(usize, usize),

    /// Set `VX = NN`
    ///
    /// Arguments: `(VX, NN)`
    /// Opcode: `6XNN`
    SetToVal(usize, u8),

    /// Add `NN` to `VX`
    /// Does not affect the carry flag.
    ///
    /// Arguments: `(VX, NN)`
    /// Opcode: `7XNN`
    AddVal(usize, u8),

    /// Set `VX = VY`
    ///
    /// Arguments: `(VX, VY)`
    /// Opcode: `8XY0`
    SetToReg(usize, usize),

    /// Bitwise OR (`VX |= VY`)
    ///
    /// Arguments: `(VX, VY)`
    /// Opcode: `8XY1`
    BitwiseOr(usize, usize),

    /// Bitwise AND (`VX &= VY`)
    ///
    /// Arguments: `(VX, VY)`
    /// Opcode: `8XY2`
    BitwiseAnd(usize, usize),

    /// Bitwise XOR (`VX ^= VY`)
    ///
    /// Arguments: `(VX, VY)`
    /// Opcode: `8XY3`
    BitwiseXor(usize, usize),

    /// Add `VY` to `VX` (`VX += VY`)
    /// Sets `VF` if carry
    ///
    /// Arguments: `(VX, VY)`
    /// Opcode: `8XY4`
    AddReg(usize, usize),

    /// Subtract `VY` from `VX` (`VX -= VY`)
    /// Clears `VF` if borrow
    ///
    /// Arguments: `(VX, VY)`
    /// Opcode: `8XY5`
    SubReg(usize, usize),

    /// Perform a single right shift on `VX`
    /// Dropped bit is stored in `VF`
    ///
    /// Arguments: `(VX)`
    /// Opcode: `8XY6`
    ShiftRight(usize),

    /// Subtract `VX` from `VY` and store it in `VX` (`VX = VY - VX`)
    ///
    /// Arguments: `(VX, VY)`
    /// Opcode: `8XY7`
    SubFromReg(usize, usize),

    /// Perform a single left shift on `VX`
    /// Overflowed bit is stored in `VF`
    ///
    /// Arguments: `(VX)`
    /// Opcode: `8XYE`
    ShiftLeft(usize),

    /// Skip an instruction if `VX != VY`
    ///
    /// Arguments: `(VX, VY)`
    /// Opcode: `9XY0`
    SkipIfRegNE(usize, usize),

    /// Set the `I` register to `NNN`
    ///
    /// Arguments: `(NNN)`
    /// Opcode: `ANNN`
    SetIndex(u16),

    /// Move the `I` register to `V0` + `NNN`
    ///
    /// Arguments: `NNN`
    /// Opcode: `BNNN`
    JumpV0Distance(u16),

    /// Set `VX` to a random number
    /// The number is then AND'ed with `NN`
    ///
    /// Arguments: `(VX, NNN)`
    /// Opcode: `CXNN`
    Rand(usize, u8),

    /// Draw a sprite to the screen
    /// Sprite data begins at the `I` register's value
    /// The sprite will be drawn at coordinates (`X`, `Y`)
    /// The sprite should have `N` rows
    ///
    /// Arguments: `(X, Y, N)`
    /// Opcode: `DXYN`
    DrawSprite(u16, u16, u8),

    /// Skip an instruction if the key index in `VX` is pressed
    ///
    /// Arguments: `(VX)`
    /// Opcode: `EX9E`
    SkipIfKeyPressed(usize),

    /// Skip an instruction if the key index in `VX` is not pressed
    ///
    /// Arguments: `(VX)`
    /// Opcode: `EXA1`
    SkipIfKeyNotPressed(usize),

    /// Set `VX` to the delay timer's current value
    ///
    /// Arguments: `(VX)`
    /// Opcode: `FX07`
    GetDelayTimer(usize),

    /// Wait until a key is pressed
    /// Once a key is pressed, it is stored in `VX`
    /// If multiple keys are pressed, the lowest-indexed key is stored
    ///
    /// Arguments: `(VX)`
    /// Opcode: `FX0A`
    WaitKey(usize),

    /// Set the delay timer's current value to `VX`
    ///
    /// Arguments: `(VX)`
    /// Opcode: `FX15`
    SetDelayTimer(usize),

    /// Set the sound timer's current value to `VX`
    ///
    /// Arguments: `(VX)`
    /// Opcode: `FX18`
    SetSoundTimer(usize),

    /// Increment the `I` register by `VX`
    /// Wraps if the value overflows
    ///
    /// Arguments: `(VX)`
    /// Opcode: `FX1E`
    IncrementI(usize),

    /// Sets the `I` register to the address of a font character
    /// for a number (`0x1` to `0xF`)
    /// The desired character is loaded from `VX`
    ///
    /// Arguments: `(VX)`
    /// Opcode: `FX29`
    LoadFontChar(usize),

    /// Load BCD (binary-encoded decimal) of `VX` into RAM
    /// The start address of this storage is the current value of the `I` register
    /// Will always store 3 bytes for the 3 digits
    ///
    /// Arguments: `(VX)`
    /// Opcode: `FX33`
    #[allow(clippy::upper_case_acronyms)]
    BCD(usize),

    /// Store `V0` -> `VX` into RAM
    /// Begins at the address stored in the `I` register
    ///
    /// Arguments: `(VX)`
    /// Opcode: `FX55`
    LoadIntoRam(usize),

    /// Store RAM into `V0` -> `VX`
    /// Begins from the address stored in the `I` register
    ///
    /// Arguments: `(VX)`
    /// Opcode: `FX65`
    LoadFromRam(usize),
}
impl Opcode {
    pub fn new(opcode: u16) -> Self {
        use Opcode::*;
        const A: u16 = 0xA;
        const B: u16 = 0xB;
        const C: u16 = 0xC;
        const D: u16 = 0xD;
        const E: u16 = 0xE;
        const F: u16 = 0xF;
        let parts = Opcode::split(opcode);
        let nn = (opcode & 0x00FF) as u8;
        let nnn = opcode & 0x0FFF;

        return match parts {
            (0,0,0,0) => Nop,
            (0,0,E,0) => ClearScreen,
            (0,0,E,E) => Return,
            (1,_,_,_) => Jump(nnn),
            (2,_,_,_) => Call(nnn),
            (3,x,_,_) => SkipIfValEQ(x as usize, nn),
            (4,x,_,_) => SkipIfValNE(x as usize, nn),
            (5,x,y,0) => SkipIfRegEQ(x as usize, y as usize),
            (6,x,_,_) => SetToVal(x as usize, nn),
            (7,x,_,_) => AddVal(x as usize, nn),
            (8,x,y,0) => SetToReg(x as usize, y as usize),
            (8,x,y,1) => BitwiseOr(x as usize, y as usize),
            (8,x,y,2) => BitwiseAnd(x as usize, y as usize),
            (8,x,y,3) => BitwiseXor(x as usize, y as usize),
            (8,x,y,4) => AddReg(x as usize, y as usize),
            (8,x,y,5) => SubReg(x as usize, y as usize),
            (8,x,_,6) => ShiftRight(x as usize),
            (8,x,y,7) => SubFromReg(x as usize, y as usize),
            (8,x,_,E) => ShiftLeft(x as usize),
            (9,x,y,0) => SkipIfRegNE(x as usize, y as usize),
            (A,_,_,_) => SetIndex(nnn),
            (B,_,_,_) => JumpV0Distance(nnn),
            (C,x,_,_) => Rand(x as usize, nn),
            (D,x,y,n) => DrawSprite(x, y, n as u8),
            (E,x,9,E) => SkipIfKeyPressed(x as usize),
            (E,x,A,1) => SkipIfKeyNotPressed(x as usize),
            (F,x,0,7) => GetDelayTimer(x as usize),
            (F,x,0,A) => WaitKey(x as usize),
            (F,x,1,5) => SetDelayTimer(x as usize),
            (F,x,1,8) => SetSoundTimer(x as usize),
            (F,x,1,E) => IncrementI(x as usize),
            (F,x,2,9) => LoadFontChar(x as usize),
            (F,x,3,3) => BCD(x as usize),
            (F,x,5,5) => LoadIntoRam(x as usize),
            (F,x,6,5) => LoadFromRam(x as usize),

            _ => unimplemented!("Nonexistent opcode {}!", opcode)
        };
    }

    fn split(opcode: u16) -> (u16,u16,u16,u16) {
        return (
            (opcode & 0xF000) >> 12,
            (opcode & 0x0F00) >> 8,
            (opcode & 0x00F0) >> 4,
            (opcode & 0x000F)
        );
    }
}
