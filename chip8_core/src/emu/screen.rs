impl super::Emu {
    pub fn get_display(&self) -> &[bool] {
        return &self.screen;
    }
}
