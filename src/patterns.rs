pub const PATTERNS: &[(&str, LangPatterns)] = &[
    ("js", LangPatterns::JS),
    ("ts", LangPatterns::TS),
    ("rs", LangPatterns::RS),
];

#[derive(Clone, Copy)]
pub struct LangPatterns {
    pub line_comment: &'static str,
    pub comment_block_start: &'static str,
    pub comment_block_end: &'static str,
    pub import: &'static str,
}

impl Default for LangPatterns {
    fn default() -> Self {
        LangPatterns {
            line_comment: "unknown",
            comment_block_start: "unknown",
            comment_block_end: "unknown",
            import: "unknown",
        }
    }
}

impl LangPatterns {
    const JS: LangPatterns = LangPatterns {
        line_comment: "//",
        comment_block_start: "/*",
        comment_block_end: "*/",
        import: "import",
    };
    const TS: LangPatterns = LangPatterns {
        line_comment: "//",
        comment_block_start: "/*",
        comment_block_end: "*/",
        import: "import",
    };
    const RS: LangPatterns = LangPatterns {
        line_comment: "//",
        comment_block_start: "/*",
        comment_block_end: "*/",
        import: "use",
    };
}
