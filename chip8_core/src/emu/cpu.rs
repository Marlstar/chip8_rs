impl super::Emu {
    pub(super) fn tick(&mut self) {
        let op = self.fetch_opcode();
    }

    fn fetch_opcode(&mut self) -> u16 {
        let higher_byte = self.ram[self.pc as usize] as u16;
        let lower_byte = self.ram[(self.pc + 1) as usize] as u16;
        // Combine the two u8's into one u16 opcode
        let opcode = (higher_byte << 8) | lower_byte;
        self.pc += 2;
        return opcode;
    }
}
