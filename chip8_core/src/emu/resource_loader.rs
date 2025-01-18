impl super::Emu {
    pub(super) fn load_font(&mut self) {
        use crate::resources::font::*;
        self.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);
    }
}
