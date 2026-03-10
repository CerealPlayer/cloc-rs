pub mod processor;

#[derive(Default)]
pub struct Count {
    pub lines: usize,
    pub empty: usize,
    pub comments: usize,
    pub imports: usize,
}

impl Count {
    pub fn add_count(&mut self, count: &Count) {
        self.lines += count.lines;
        self.comments += count.comments;
        self.empty += count.empty;
        self.imports += count.imports;
    }
}
