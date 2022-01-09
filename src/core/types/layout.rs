#[derive(Debug, Clone)]
pub struct Layout {
    pub offset: usize,
    pub size: usize,
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

    pub fn last(&self) -> usize {
        self.offset + self.size
    }
}
