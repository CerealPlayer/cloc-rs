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

// Static patterns – zero runtime allocation!
const PATTERNS: &[(&str, LangPatterns)] = &[
    ("js", LangPatterns::JS),
    ("ts", LangPatterns::TS),
    ("rs", LangPatterns::RS),
];

#[derive(Clone, Copy, Default)]
struct LangPatterns {
    line_comment: &'static str,
    block_start: &'static str,
    block_end: &'static str,
    import: &'static str,
}

impl LangPatterns {
    const JS: LangPatterns = LangPatterns {
        line_comment: "//",
        block_start: "/*",
        block_end: "*/",
        import: "import",
    };
    const TS: LangPatterns = LangPatterns {
        line_comment: "//",
        block_start: "/*",
        block_end: "*/",
        import: "import",
    };
    const RS: LangPatterns = LangPatterns {
        line_comment: "//",
        block_start: "/*",
        block_end: "*/",
        import: "use",
    };
}

pub fn process_file(ext: &str, text: String) -> Count {
    let patterns = PATTERNS
        .iter()
        .find(|(e, _)| ext == *e)
        .map(|(_, p)| *p)
        .unwrap_or_default(); // or handle NONE

    let mut count = Count::default();
    let mut in_block = false;

    for line in text.lines() {
        count.lines += 1;
        let trimmed = line.trim();

        if trimmed.is_empty() {
            count.empty += 1;
            continue;
        }

        if in_block {
            count.comments += 1;
        } else if line.contains(patterns.line_comment) {
            count.comments += 1;
        } else if line.contains(patterns.import) {
            count.imports += 1;
        }

        // Block comments (multi-line aware)
        if line.contains(patterns.block_start) {
            in_block = true;
            count.comments += 1;
        }
        if line.contains(patterns.block_end) {
            in_block = false;
            count.comments += 1;
        }
    }
    count
}
