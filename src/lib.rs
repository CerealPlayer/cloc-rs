mod patterns;

use patterns::PATTERNS;

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

pub fn process_file(ext: &str, text: String) -> Count {
    let patterns = PATTERNS
        .iter()
        .find(|(e, _)| ext == *e)
        .map(|(_, p)| *p)
        .unwrap_or_default(); // or handle NONE

    let mut count = Count::default();
    let mut in_comment_block = false;

    for line in text.lines() {
        count.lines += 1;
        let trimmed = line.trim();

        if trimmed.is_empty() {
            count.empty += 1;
            continue;
        }

        if in_comment_block {
            count.comments += 1;
        } else if trimmed.starts_with(patterns.line_comment) {
            count.comments += 1;
        } else if trimmed.starts_with(patterns.import) {
            count.imports += 1;
        }

        if line.contains(patterns.comment_block_start) {
            in_comment_block = true;
            count.comments += 1;
        }
        if line.contains(patterns.comment_block_end) {
            in_comment_block = false;
            count.comments += 1;
        }
    }
    count
}
