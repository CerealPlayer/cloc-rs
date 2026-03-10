use crate::{
    Count,
    processor::{LangProcessor, js::JsProcessor},
};

pub struct HtmlProcessor {
    in_script: bool,
    in_style: bool,
    script_processor: JsProcessor,
}

impl HtmlProcessor {
    pub fn new() -> Self {
        Self {
            in_script: false,
            in_style: false,
            script_processor: JsProcessor::new(),
        }
    }
}

impl LangProcessor for HtmlProcessor {
    fn count(&mut self, text: &str) -> Count {
        let mut c = Count::default();
        let mut buffer = String::new();

        for line in text.lines() {
            c.lines += 1;
            let trimmed = line.trim();

            if trimmed.is_empty() {
                c.empty += 1;
                continue;
            }

            // Simple tag detection (naive but fast)
            if trimmed.starts_with("<!--") {
                c.comments += 1; // HTML comments
                continue;
            }

            if trimmed.starts_with("<script") {
                self.in_script = true;
                continue;
            }
            if trimmed.starts_with("</script>") {
                self.in_script = false;
                // Count inline JS
                c.comments += self.script_processor.count(&buffer).comments;
                buffer.clear();
                continue;
            }

            if trimmed.starts_with("<style") {
                self.in_style = true;
                continue;
            }
            if trimmed.starts_with("</style>") {
                self.in_style = false;
                buffer.clear();
                continue;
            }

            // Buffer content for inline script/style
            if self.in_script {
                buffer.push_str(line);
                buffer.push('\n');
            } else if self.in_style {
                // CSS counting below
                if trimmed.starts_with("/*") || trimmed.starts_with("//") {
                    c.comments += 1;
                }
            }
        }
        c
    }
}
