use crate::{Count, processor::LangProcessor};

pub struct JsProcessor {
    in_block_comment: bool,
    in_multi_import: bool,
}

impl JsProcessor {
    pub fn new() -> Self {
        Self {
            in_block_comment: false,
            in_multi_import: false,
        }
    }
}

impl LangProcessor for JsProcessor {
    fn count(&mut self, text: &str) -> Count {
        let mut c = Count::default();

        for line in text.lines() {
            c.lines += 1;
            let trimmed = line.trim();

            if trimmed.is_empty() {
                c.empty += 1;
                continue;
            }

            // Block comments /* */
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

            // Line comments //
            if trimmed.starts_with("//") {
                c.comments += 1;
                continue;
            }

            // IMPORTS (single + multi-line)
            if !self.in_block_comment {
                if trimmed.starts_with("import ") {
                    self.in_multi_import = trimmed.contains('{'); // { → multi-line
                    c.imports += 1;
                } else if self.in_multi_import && trimmed.ends_with(';') {
                    self.in_multi_import = false;
                }
            }
        }
        c
    }
}
