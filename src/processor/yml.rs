use crate::{Count, processor::LangProcessor};

pub struct YamlProcessor;

impl YamlProcessor {
    pub fn new() -> Self {
        Self
    }
}

impl LangProcessor for YamlProcessor {
    fn count(&mut self, text: &str) -> Count {
        let mut c = Count::default();

        for line in text.lines() {
            c.lines += 1;
            let trimmed = line.trim();

            if trimmed.is_empty() {
                c.empty += 1;
                continue;
            }

            if trimmed.starts_with('#') {
                c.comments += 1;
                continue;
            }

            if trimmed.contains("include:")
                || trimmed.contains("extends:")
                || trimmed.contains("$ref:")
            {
                c.imports += 1;
            }
        }
        c
    }
}
