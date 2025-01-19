impl super::Emu {
    pub fn tick_timers(&mut self) {
        if self.dt > 0 { self.dt -= 1; }
        if self.st > 0 {
            if self.st == 1 {
                Self::beep();
            }
            self.st -= 1;
        }
    }
}
