impl super::Emu {
    pub fn keypress(&mut self, idx: usize, pressed: bool) {
        if idx >= super::NUM_KEYS { return };
        self.keys[idx] = pressed;
    }
}
