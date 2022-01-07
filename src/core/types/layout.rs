pub struct Layout {
    offset: usize,
    size: usize,
}

impl Layout {
    pub fn new(off: usize) -> Self {
        Layout {
            offset: off,
            size: 0,
        }
    }

    pub fn end(&mut self, off: usize) {
        self.size = off - self.offset;
    }
}