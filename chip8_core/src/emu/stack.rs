impl super::Emu {
    pub(super) fn push(&mut self, val: u16) {
        if self.sp as usize == crate::constants::STACK_SIZE - 1 { panic!("stack pointer overflow") }
        self.stack[self.sp as usize] = val;
        self.sp += 1;
    }

    pub(super) fn pop(&mut self) -> u16 {
        if self.sp == 0 { panic!("stack pointer underflow"); }
        self.sp -= 1;
        return self.stack[self.sp as usize];
    }
}
