#[derive(Default)]
pub struct CycleVec<T> {
    data: Vec<T>,
    cursor: usize,
}

impl<T> CycleVec<T> {
    pub fn new(data: Vec<T>) -> Self {
        Self { data, cursor: 0 }
    }

    pub fn cursor(&self) -> usize {
        self.cursor
    }

    pub fn next(&mut self) {
        self.cursor = (self.cursor + 1) % self.data.len();
    }

    pub fn prev(&mut self) {
        if self.cursor == 0 {
            self.cursor = self.data.len() - 1;
        } else {
            self.cursor -= 1;
        }
    }

    pub fn current(&self) -> &T {
        &self.data[self.cursor]
    }
}
