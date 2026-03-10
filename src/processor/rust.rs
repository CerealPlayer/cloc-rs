use crate::{Count, processor::LangProcessor};

pub struct RustProcessor {
    in_block_comment: bool,
}

impl RustProcessor {
    pub fn new() -> Self {
        Self {
            in_block_comment: false,
        }
    }
}

impl LangProcessor for RustProcessor {
    fn count(&mut self, text: &str) -> Count {
        let mut c = Count::default();

        for line in text.lines() {
            c.lines += 1;
            let trimmed = line.trim();

            if trimmed.is_empty() {
                c.empty += 1;
                continue;
            }

            if self.in_block_comment {
                c.comments += 1;
                if trimmed.contains("*/") {
                    self.in_block_comment = false;
                }
                continue;
            }

            if trimmed.starts_with("/*") {
                self.in_block_comment = true;
                c.comments += 1;
                continue;
            }

            if trimmed.starts_with("//") {
                c.comments += 1;
                continue;
            }

            if trimmed.starts_with("use ") {
                c.imports += 1;
            }
        }

        c
    }
}
