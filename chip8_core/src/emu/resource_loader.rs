impl super::Emu {
    pub(super) fn load_font(&mut self) {
        use crate::resources::font::*;
        self.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);
    }

    pub fn load_rom(&mut self, data: &[u8]) {
        let start = super::START_ADDR as usize;
        let end = start + data.len();
        self.ram[start..end].copy_from_slice(data);
    }
}
