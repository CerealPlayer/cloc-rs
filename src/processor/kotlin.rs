use crate::{Count, processor::LangProcessor};

pub struct KotlinProcessor {
    in_block_comment: bool,
    in_javadoc: bool,
}

impl KotlinProcessor {
    pub fn new() -> Self {
        Self {
            in_block_comment: false,
            in_javadoc: false,
        }
    }
}

impl LangProcessor for KotlinProcessor {
    fn count(&mut self, text: &str) -> Count {
        let mut c = Count::default();

        for line in text.lines() {
            c.lines += 1;
            let trimmed = line.trim();

            if trimmed.is_empty() {
                c.empty += 1;
                continue;
            }

            if self.in_javadoc {
                c.comments += 1;
                if trimmed.contains("*/") {
                    self.in_javadoc = false;
                }
                continue;
            }

            if self.in_block_comment {
                c.comments += 1;
                if trimmed.contains("*/") {
                    self.in_block_comment = false;
                }
                continue;
            }

            if trimmed.starts_with("/**") {
                self.in_javadoc = true;
                c.comments += 1;
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

            if trimmed.starts_with("import ") {
                c.imports += 1;
            }
        }
        c
    }
}
